use sqlparser::dialect::{GenericDialect, MySqlDialect, AnsiDialect};
use sqlparser::parser::{Parser, ParserOptions};

fn main() {
    let dialect = MySqlDialect {}; // or AnsiDialect

    let sql = 
    r#"-- 结算流程执行状态表
    CREATE TABLE tb_distribute_settlement_state (
      `id`                 int(11) unsigned NOT NULL  AUTO_INCREMENT                 COMMENT 'id',
      UNIQUE KEY `task_uniq` (`ds`, `dag_name`, `pair`,`state`)
    ) ENGINE = InnoDB DEFAULT CHARSET = utf8 -- COMMENT = '结算流程执行状态表'
    "#;


    let ast = Parser::parse_sql(&dialect, sql).unwrap();

    println!("AST: {:#?}", ast);
}
