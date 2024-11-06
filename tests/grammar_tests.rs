use pest::Parser;
use anyhow::anyhow;
use pest_01::*;

#[test]
fn basic_test() -> anyhow::Result<()> {
    /// Тестуємо простий числовий вхід
    let pair = Grammar::parse(Rule::field, "-273.15")?.next().ok_or_else( 
        || anyhow!("no pair"))?;


    assert_eq!(pair.as_str(), "-273.15");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 7);

    /// Тестуємо неправильний вхід
    let pair = Grammar::parse(Rule::field, "x");
    assert!(pair.is_err());

    /// Тестуємо порожній вхід
    let pair = Grammar::parse(Rule::field, "");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn record_test() -> anyhow::Result<()> {
    /// Тестуємо правильний формат запису для правила
    let pair = Grammar::parse(Rule::record, "-273.15,99")?
    .next()
    .ok_or_else( || anyhow!("no pair"))?;
    
    
    assert_eq!(pair.as_str(), "-273.15,99");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 10);
    
    /// Тестуємо неправильний вхід
    let pair = Grammar::parse(Rule::record, "x");
    assert!(pair.is_err());
    
    /// Тестуємо порожній вхід
    let pair = Grammar::parse(Rule::record, "");
    assert!(pair.is_err());
    
    Ok(())
}





#[test]
fn file_test() -> anyhow::Result<()> {
    /// Тестуємо файл з правильним входом
    let pair = Grammar::parse(Rule::file, "-273.15,99\n")?
    .next()
    .ok_or_else( || anyhow!("no pair"))?;
    
    assert_eq!(pair.as_str(), "-273.15,99\n");
    
    /// Тестуємо неправильний вхід
    let pair = Grammar::parse(Rule::file, "x");
    assert!(pair.is_err());
    
    /// тест на некоректний формат
    let pair = Grammar::parse(Rule::file, "-273.15 99\n");
    assert!(pair.is_err());

    /// тест на відсутність нового рядка
    let pair = Grammar::parse(Rule::file, "-273.15,99,85,98.0");
    assert!(pair.is_err());

    Ok(())
}



#[test]
fn file_test2() -> anyhow::Result<()> {
    /// Тестуємо файл з правильним входом
    let pair = Grammar::parse(Rule::file, "-273.15,99\n36.8,98.1\n")?
    .next()
    .ok_or_else( || anyhow!("no pair"))?;
    
    assert_eq!(pair.as_str(), "-273.15,99\n36.8,98.1\n");

    Ok(())
}
    
