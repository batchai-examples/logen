use crate::{def::AppD, formatter::Formatter, template::Template};
use anyhow::Result;

use rand::Rng;

use super::{timestamp::Timestamp, logger::Logger, line::Line};

pub struct App<'a> {
    def: &'a AppD,
    formatter: Box<dyn Formatter + 'a>,
    template: Template,
    logger: Vec<Logger<'a>>,
}

impl <'a> App<'a> {

    pub fn new(def: &'a AppD) -> Result<App<'a>> {
        let mut t = Template::new();
        def.post_init(&mut t)?;

        Ok(App {
            def,
            formatter: def.formatter.new_formatter(),
            template: t,
            logger: {
                let mut v = Vec::new();
                for (i, logger_d) in def.logger.iter().enumerate() {
                    let logger_id = format!("{}/{}", def.name, i);
                    v.push(Logger::new(logger_d, logger_id));
                }
                v
            },
        })
    }

    pub fn name(&self) -> &str {
        &self.def.name
    }

    fn choose_logger(&self) -> &Logger {
        let mut k = 0;
        let max = self.logger.len();
        let mut rng = rand::thread_rng();

        while k < 10 {
            let i = rng.gen_range(0..max * 2);
            if i < max {
                return &self.logger[i];
            }

            k = k+1;
        }

        return &self.logger[0];
    }

    pub fn generate(&mut self) -> Result<()> {
        let d = self.def;
        let f = self.formatter.as_ref();
        let mut ts = Timestamp::new(&d.timestamp, d.num_of_lines);

        for i in 0..d.num_of_lines {
            ts.inc();

            let mut ln = Line::new(i, self, &self.template, &ts);
            f.prepare(&mut ln);
            self.choose_logger().render(&mut ln)?;

            println!("{}", f.format(&ln)?);
        }

        Ok(())
    }

}
