use arboard::Clipboard;
use regex::Regex;

fn main() {

    let mut clipboard = Clipboard::new().unwrap();

    let text = clipboard.get_text().unwrap();

    let corrections = get_corrections();

    let result = process_text(&text, &corrections);

    clipboard.set_text(result).unwrap();
}

fn get_corrections() -> Vec<(&'static str, &'static str)> {
    vec![
        ("вызватьисключение", "ВызватьИсключение"),
        ("возврат", "Возврат"),
        ("выполнить", "Выполнить"),
        ("добавитьобработчик", "ДобавитьОбработчик"),
        ("для", "Для"),
        ("если", "Если"),
        ("знач", "Знач"),
        ("и", "И"),
        ("из", "Из"),
        ("или", "Или"),
        ("иначе", "Иначе"),
        ("иначеесли", "ИначеЕсли"),
        ("исключение", "Исключение"),
        ("истина", "Истина"),
        ("каждого", "Каждого"),
        ("конецесли", "КонецЕсли"),
        ("конецпопытки", "КонецПопытки"),
        ("конецпроцедуры", "КонецПроцедуры"),
        ("конецфункции", "КонецФункции"),
        ("конеццикла", "КонецЦикла"),
        ("не", "Не"),
        ("неопределено", "Неопределено"),
        ("перейти", "Перейти"),
        ("перем", "Перем"),
        ("по", "По"),
        ("пока", "Пока"),
        ("попытка", "Попытка"),
        ("процедура", "Процедура"),
        ("прервать", "Прервать"),
        ("продолжить", "Продолжить"),
        ("тогда", "Тогда"),
        ("цикл", "Цикл"),
        ("удалитьобработчик", "УдалитьОбработчик"),
        ("функция", "Функция"),
        ("экспорт", "Экспорт"),
        ("вебклиент", "ВебКлиент"),
        ("внешнеесоединение", "ВнешнееСоединение"),
        ("клиент", "Клиент"),
        ("мобильноеприложениеклиент", "МобильноеПриложениеКлиент"),
        ("мобильноеприложениесервер", "МобильноеПриложениеСервер"),
        ("мобильныйклиент", "МобильныйКлиент"),
        ("наклиенте", "НаКлиенте"),
        ("насервере", "НаСервере"),
        ("область", "Область"),
        ("конецобласти", "КонецОбласти"),
        ("сервер", "Сервер"),
        (
            "толстыйклиентобычноеприложение",
            "ТолстыйКлиентОбычноеПриложение",
        ),
        (
            "толстыйклиентуправляемоеприложение",
            "ТолстыйКлиентУправляемоеПриложение",
        ),
        ("тонкийклиент", "ТонкийКлиент"),
        (
            "наклиентенасерверебезконтекста",
            "НаКлиентеНаСервереБезКонтекста",
        ),
        ("наклиентенасервере", "НаКлиентеНаСервере"),
    ]
}

fn process_line(line: &str, corrections: &[(&str, &str)]) -> String {
    if let Some(comment_post) = line.find("//") {
        let (code, comment) = line.split_at(comment_post);
        format!("{}{}", process_code(code, corrections), comment)
    } else {
        process_code(line, corrections)
    }

}

fn process_code(code: &str, corrections: &[(&str, &str)]) -> String {
    let mut result = code.to_string();
    for (wrong, correct) in corrections {
        let pattern = format!(r"(?i)\b{}\b", regex::escape(wrong));
        let re = Regex::new(&pattern).unwrap();
        result = re.replace_all(&result, *correct).to_string();
    }
    result 
}

fn process_text(text: &str, corrections: &[(&str, &str)]) -> String {
    text.lines()
        .map(|line| process_line(line, corrections))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process_line() {
        let corrections = vec![("если", "Если"), ("тогда", "Тогда")];
        assert_eq!(
            process_line("если условие тогда", &corrections),
            "Если условие Тогда"
        );
    }
    
    #[test]
    fn test_process_line_with_comment() {
        let corrections = vec![("если", "Если"), ("тогда", "Тогда")];
        assert_eq!(
            process_line("если условие тогда // если тогда", &corrections),
            "Если условие Тогда // если тогда"
        );
    }

}