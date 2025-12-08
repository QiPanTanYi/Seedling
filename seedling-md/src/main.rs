use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    if let Err(e) = run() {
        eprintln!("运行失败: {}", e);
        std::process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let exe_dir = current_exe_dir()?;
    let md_files = list_md_files(&exe_dir)?;

    for file in md_files {
        if let Err(err) = process_md_file(&file) {
            eprintln!("处理文件失败: {} -> {}", file.display(), err);
        }
    }

    Ok(())
}

fn current_exe_dir() -> io::Result<PathBuf> {
    let exe = env::current_exe()?;
    Ok(exe
        .parent()
        .map(|p| p.to_path_buf())
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "无法获取可执行文件所在目录"))?)
}

fn list_md_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if path
                .extension()
                .and_then(OsStr::to_str)
                .map(|ext| ext.eq_ignore_ascii_case("md"))
                .unwrap_or(false)
            {
                files.push(path);
            }
        }
    }
    Ok(files)
}

fn process_md_file(path: &Path) -> io::Result<()> {
    let mut content = fs::read_to_string(path)?;

    let bom = content.starts_with('\u{feff}');
    if bom {
        // 保留 BOM，但在处理时跳过
        content = content.trim_start_matches('\u{feff}').to_string();
    }

    let (done, todo, total) = count_tasks(&content);
    if total == 0 {
        // 无待办任务，不插入
        return Ok(());
    }

    let header = format_header_line(done, todo, total);
    let line_ending = detect_line_ending(&content);
    let newline = match line_ending {
        LineEnding::CRLF => "\r\n",
        LineEnding::LF => "\n",
    };

    let mut new_content = if starts_with_stats_header(&content) {
        replace_first_line(&content, &header, newline)
    } else {
        format!("{}{}{}", header, newline, content)
    };

    // 尾行插入/更新
    new_content = upsert_last_line(&new_content, &header, newline);

    let final_content = if bom {
        format!("\u{feff}{}", new_content)
    } else {
        new_content
    };

    fs::write(path, final_content)?;
    Ok(())
}

fn count_tasks(content: &str) -> (usize, usize, usize) {
    let mut done = 0usize;
    let mut todo = 0usize;

    for line in content.lines() {
        let l = line.trim_start();
        let is_bullet = l.starts_with("- [") || l.starts_with("* [");
        if !is_bullet { continue; }

        let bytes = l.as_bytes();
        // 查找 '[' 的索引
        if let Some(idx) = l.find('[') {
            let mark = bytes.get(idx + 1).copied();
            match mark {
                Some(b'x') | Some(b'X') => done += 1,
                Some(b' ') => todo += 1,
                _ => {}
            }
        }
    }

    (done, todo, done + todo)
}

fn format_header_line(done: usize, todo: usize, total: usize) -> String {
    let bar = make_progress_bar(done, total);
    let todo_u = format!("<u>{}</u>", todo);
    let line = format!(
        "{} ⚡ 今日进度 {}/{} | ⏳ 未完成 {} | ✅ 已完成 {} · _By Seedling_ 🌱",
        bar, done, total, todo_u, done
    );
    format!("**{}**", line)
}

fn starts_with_stats_header(s: &str) -> bool {
    let first = s.lines().next().map(|l| l.trim()).unwrap_or("");
    is_stats_line(first)
}

fn ends_with_stats_header(s: &str) -> bool {
    let last_nonempty = s
        .lines()
        .rev()
        .find(|l| !l.trim().is_empty())
        .map(|l| l.trim())
        .unwrap_or("");
    is_stats_line(last_nonempty)
}

fn is_stats_line(l: &str) -> bool {
    let t = l.trim();
    let t = t.trim_start_matches('*').trim_end_matches('*');
    let t = t.replace('*', "").replace('_', "");
    t.starts_with("今日未完成目标 ") || t.contains("今日进度") || t.contains("By Seedling")
}

#[derive(Copy, Clone)]
enum LineEnding { CRLF, LF }

fn detect_line_ending(s: &str) -> LineEnding {
    if s.contains("\r\n") { LineEnding::CRLF } else { LineEnding::LF }
}

fn replace_first_line(content: &str, header: &str, newline: &str) -> String {
    // 查找第一行结尾
    let mut chars = content.char_indices();
    while let Some((i, ch)) = chars.next() {
        if ch == '\n' {
            // 处理可能的 CRLF
            let start_next = i + 1;
            if i > 0 && &content[i-1..=i] == "\r\n" {
                let after = &content[start_next..];
                return format!("{}{}{}", header, newline, after);
            } else {
                let after = &content[start_next..];
                return format!("{}{}{}", header, newline, after);
            }
        }
    }
    // 没有换行，整文件为一行（替换首行为header）
    header.to_string()
}

fn upsert_last_line(content: &str, header: &str, newline: &str) -> String {
    let base = content.trim_end_matches(['\r', '\n']).to_string();
    if ends_with_stats_header(&base) {
        if let Some((before, _last)) = base.rsplit_once('\n') {
            return format!("{}{}{}", before, newline, header);
        } else {
            return header.to_string();
        }
    } else {
        format!("{}{}{}", base, newline, header)
    }
}

fn make_progress_bar(done: usize, total: usize) -> String {
    // 10格进度条，使用全角/半角方块增强可视性
    let width: usize = 10;
    let filled = if total == 0 { 0 } else { ((done as f32 / total as f32) * width as f32).round() as usize };
    let filled = filled.min(width);
    let empty = width - filled;
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_tasks() {
        let s = "- [x] 国际化i18n\n- [ ] 前端写一个数据库MCP聊天页面，用之前那个mysql-mcp\n- [ ] 增加生成小程序码的功能";
        let (done, todo, total) = count_tasks(s);
        assert_eq!((done, todo, total), (1, 2, 3));
        let header = format_header_line(done, todo, total);
        assert!(header.starts_with("**"));
        assert!(header.ends_with("**"));
        assert!(header.contains("_By Seedling_"));
        assert!(header.contains("⏳ 未完成 <u>2</u>"));
        let pos_unfinished = header.find("⏳ 未完成").unwrap();
        let pos_done = header.find("✅ 已完成 1").unwrap();
        assert!(pos_unfinished < pos_done);
    }

    #[test]
    fn test_replace_or_insert() {
        let original = "今日未完成目标 0 ，已完成目标 0（总目标 0/0）\n- [x] a\n- [ ] b";
        let header = format_header_line(1,1,2);
        let res = replace_first_line(original, &header, "\n");
        assert!(res.starts_with(&header));
        assert!(res.contains("- [x] a"));
    }

    #[test]
    fn test_upsert_last_line() {
        let body = "- [x] a\n- [ ] b";
        let header = format_header_line(1,1,2);
        let content = format!("{}\n{}", header, body);
        let appended = upsert_last_line(&content, &header, "\n");
        assert!(appended.ends_with(&header));

        // 替换尾部已有统计行
        let with_tail = format!("{}\n{}\n{}", header, body, header);
        let replaced = upsert_last_line(&with_tail, &format_header_line(1,0,1), "\n");
        assert!(ends_with_stats_header(&replaced));
    }
}
