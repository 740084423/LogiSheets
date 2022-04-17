use parser::ast;

use super::{CalcValue, Value};

pub enum ConditionResult {
    True,
    False,
    Error(ast::Error),
}

pub fn get_condition_result(calc_value: CalcValue) -> ConditionResult {
    match calc_value {
        CalcValue::Scalar(s) => match s {
            Value::Blank => ConditionResult::False,
            Value::Number(num) => {
                if num.abs() < 1e-10 {
                    ConditionResult::False
                } else {
                    ConditionResult::True
                }
            }
            Value::Text(_) => ConditionResult::Error(ast::Error::Value),
            Value::Boolean(b) => {
                if b {
                    ConditionResult::True
                } else {
                    ConditionResult::False
                }
            }
            Value::Error(e) => ConditionResult::Error(e),
            Value::Date(_) => ConditionResult::True,
        },
        CalcValue::Union(_) => ConditionResult::Error(ast::Error::Value),
        // WPS neither support this.
        CalcValue::Range(_) => ConditionResult::Error(ast::Error::Value),
        CalcValue::Cube(_) => ConditionResult::Error(ast::Error::Ref),
    }
}

pub fn convert_f64(value: Value) -> Result<f64, ast::Error> {
    match value {
        Value::Blank => Ok(0_f64),
        Value::Number(n) => Ok(n),
        Value::Text(t) => match t.parse::<f64>() {
            Ok(n) => Ok(n),
            Err(_) => Err(ast::Error::Value),
        },
        Value::Boolean(b) => {
            if b {
                Ok(1.)
            } else {
                Ok(0.)
            }
        }
        Value::Error(e) => Err(e),
        Value::Date(_) => todo!(),
    }
}

pub fn assert_scalar_value(v: CalcValue) -> Option<Value> {
    match v {
        CalcValue::Scalar(v) => Some(v),
        _ => None,
    }
}

pub fn is_error(value: &CalcValue) -> bool {
    match value {
        CalcValue::Scalar(s) => match s {
            Value::Error(_) => true,
            _ => false,
        },
        CalcValue::Range(_) => false,
        CalcValue::Cube(_) => true,
        CalcValue::Union(_) => todo!(),
    }
}

#[cfg(test)]
pub mod tests_utils {
    use controller_base::async_func::{AsyncCalcResult, AsyncFuncCommitTrait, Task};
    use controller_base::get_active_sheet::GetActiveSheetTrait;
    use controller_base::get_curr_addr::GetCurrAddrTrait;
    use controller_base::set_curr_cell::SetCurrCellTrait;
    use parser::ast;

    use crate::calc_engine::calculator::calc_vertex::{CalcValue, CalcVertex};
    use crate::calc_engine::connector::Connector;
    use crate::vertex_manager::vertex::FormulaId;

    pub struct TestFetcher {}
    impl AsyncFuncCommitTrait for TestFetcher {
        fn query_or_commit_task(
            &mut self,
            sheet_id: controller_base::SheetId,
            cell_id: controller_base::CellId,
            task: Task,
        ) -> Option<AsyncCalcResult> {
            None
        }
    }
    impl GetActiveSheetTrait for TestFetcher {
        fn get_active_sheet(&self) -> controller_base::SheetId {
            todo!()
        }
    }

    impl GetCurrAddrTrait for TestFetcher {
        fn get_curr_addr(&self) -> controller_base::Addr {
            todo!()
        }
    }

    impl SetCurrCellTrait for TestFetcher {
        fn set_curr_cell(
            &mut self,
            active_sheet: controller_base::SheetId,
            addr: controller_base::Addr,
        ) {
            todo!()
        }
    }

    impl Connector for TestFetcher {
        fn convert(&mut self, _: &ast::CellReference) -> CalcVertex {
            todo!()
        }

        fn get_calc_value(&mut self, vertex: CalcVertex) -> CalcValue {
            match vertex {
                CalcVertex::Value(v) => v,
                CalcVertex::Reference(_) => panic!(),
                CalcVertex::Union(_) => todo!(),
            }
        }

        fn get_text(&self, _: &controller_base::TextId) -> Option<String> {
            todo!()
        }

        fn get_func_name(&self, _: &controller_base::FuncId) -> Option<String> {
            todo!()
        }

        fn get_cell_idx(
            &mut self,
            sheet_id: controller_base::SheetId,
            cell_id: &controller_base::CellId,
        ) -> Option<(usize, usize)> {
            Some((0, 0))
        }

        fn get_cell_id(
            &mut self,
            sheet_id: controller_base::SheetId,
            row: usize,
            col: usize,
        ) -> Option<controller_base::CellId> {
            todo!()
        }

        fn commit_calc_values(
            &mut self,
            vertex: FormulaId,
            result: CalcValue,
        ) -> std::collections::HashSet<crate::vertex_manager::vertex::FormulaId> {
            todo!()
        }

        fn is_async_func(&self, func_name: &str) -> bool {
            false
        }
    }
}