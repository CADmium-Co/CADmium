use std::fs;

// CSS from https://developer.mozilla.org/en-US/docs/Web/HTML/Element/table
const HTML_HEAD: &str = r#"
        <style>
table {
  border-collapse: collapse;
  border: 2px solid rgb(140 140 140);
  font-family: sans-serif;
  font-size: 0.8rem;
  letter-spacing: 1px;
}
thead,
tfoot {
  background-color: rgb(228 240 245);
}

th,
td {
  border: 1px solid rgb(160 160 160);
  padding: 8px 10px;
}

td:last-of-type {
  text-align: center;
}

tbody > tr:nth-of-type(even) {
  background-color: rgb(237 238 242);
}
        </style>
        <table>
            <thead>
                <tr>
                    <th>Selector</th>
                    <th>Case</th>
                    <th>Before</th>
                    <th>After</th>
                </tr>
            </thead>
            <tbody>
"#;
const HTML_FOOT: &str = "</tbody></table>";

pub fn save_report_html(results: Vec<(String, String, String)>) {
    let mut report = HTML_HEAD.to_string();

    for (selector, case, name) in results.iter() {
        report.push_str(&format!(r#"
            <tr>
                <td>{selector}</td>
                <td>{case}</td>
                <td><a href="{name}_before.svg"><img width="200" src="{name}_before.svg" /></a></td>
                <td><a href="{name}_after.svg"><img width="200" src="{name}_after.svg" /></a></td>
            </tr>
        "#))
    }

    report.push_str(HTML_FOOT);
    fs::write("bench-faceselector-report/index.html", report).unwrap();
}
