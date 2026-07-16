use crate::Report;

/// Компоновщик отчётов, использующий статический полиморфизм (Generics).
pub struct ReportBuilder {
    reports: Vec<String>,
}

impl ReportBuilder {
    /// Создаёт нового строителя отчётов.
    pub fn new() -> Self {
        ReportBuilder {
            reports: Vec::new(),
        }
    }

    /// Добавляет отчёт от любого объекта, реализующего трейт `Report`.
    pub fn add_report<T: Report>(mut self, item: &T) -> Self {
        self.reports.push(item.report());
        self
    }

    /// Выводит все собранные отчёты в стандартный вывод.
    pub fn report(&self) {
        for report in &self.reports {
            println!("{}", report);
        }
    }
}

impl Default for ReportBuilder {
    fn default() -> Self {
        Self::new()
    }
}
