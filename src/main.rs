use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    App = {{App}} {
        ui: <DesktopWindow>{
            show_bg: true
            width: Fill,
            height: Fill

            draw_bg: {
                fn pixel(self) -> vec4 {
                    return mix(#7, #3, #7);
                }
            }

            <View>{

                    flow: Down,
                    spacing: 20,
                    align: {
                        x: 0.5,
                        y: 0.5

                },
                button1 = <Button> {
                    draw_icon:{
                        svg_path:"M7399.39,1614.16C7357.53,1615.77 7324.04,1650.26 7324.04,1692.51C7324.04,1702.28 7316.11,1710.22 7306.33,1710.22C7296.56,1710.22 7288.62,1702.28 7288.62,1692.51C7288.62,1630.8 7337.85,1580.49 7399.14,1578.74L7389.04,1569.44C7381,1562.04 7380.49,1549.51 7387.88,1541.47C7395.28,1533.44 7407.81,1532.92 7415.85,1540.32L7461.76,1582.58C7465.88,1586.37 7468.2,1591.73 7468.15,1597.32C7468.1,1602.91 7465.68,1608.23 7461.5,1611.94L7415.59,1652.71C7407.42,1659.97 7394.9,1659.23 7387.65,1651.06C7380.39,1642.89 7381.14,1630.37 7389.3,1623.12L7399.39,1614.16Z"
                    }
                    icon_walk:{margin:{left:10}, width:16,height:Fit}
                    text: "Click to count"
                }
                input1 = <TextInput> {
                    width: 100, height: 30
                    text: "Click to count"
                }

                label1 = <Label> {
                    draw_text: {
                        color: #f
                    },
                    text: "Counter: 0"
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live)]
pub struct App {
    #[live]
    ui: WidgetRef,

    #[rust]
    counter: usize,
}

impl LiveHook for App {
    fn before_live_design(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl App {
    async fn _do_network_request(_cx: CxRef, _ui: WidgetRef, _url: &str) -> String {
        "".to_string()
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Draw(event) = event {
            return self.ui.draw_widget_all(&mut Cx2d::new(cx, event));
        }

        let actions = self.ui.handle_widget_event(cx, event);

        if self.ui.button(id!(button1)).clicked(&actions) {
            self.counter += 1;

            let label = self.ui.label(id!(label1));
            label.set_text_and_redraw(cx, &format!("Counter: {}", self.counter));
        }
    }
}

fn main() {
    app_main()
}
