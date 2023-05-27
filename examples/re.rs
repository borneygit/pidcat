use regex::Regex;

fn main() {
    let it = [
        "[ 05-25 02:35:13.081   993:  993 D/[h4m1][MainActivity] ]",
        "[ 05-24 05:54:17.377   181:  181 I/vold     ]",
        "[ 05-26 08:35:39.945   526:10355 I/chatty   ]",
        "[ 05-24 07:58:15.003  1454: 1454 W/System.err ]",
        "[ 05-24 07:59:24.038   530: 4634 D/NetworkMonitor/NetworkAgentInfo [WIFI () - 100] ]",
        "[ 05-26 08:56:42.315   526:10746 D/NetworkMonitor/NetworkAgentInfo [MOBILE (LTE) - 101] ]",
        "[ 05-24 07:59:56.939   530:  635 D/ConnectivityService ]",
        "[ 05-24 08:08:52.510   243: 1165 D/AT-RIL   ]",
        "[ 05-24 08:08:18.573     0:    0 D/ieee80211 phy0 ]",
        "[ 05-24 08:09:00.064   530:  558 I/am_pss   ]",
        "[ 05-24 05:54:17.377   181:  181 I/vold     ]",
        "[546,1000,system,89723904,77209600,0]",
        "[3849]> SIGNAL_STRENGTH [SUB0]",
        "[ 05-26 05:45:32.733  1440: 1440 W/System.err ]"
    ];
    let re = Regex::new(r"\[ (\d{2}-\d{2})\s(\d{2}:\d{2}:\d{2}\.\d{3})\s+(\d+):(.*) ]").unwrap();

    for s in it {
        if let Some(cap) = re.captures(s) {
            println!("{}---->{};", s, re.is_match(s));
            println!(">>date:{};", cap.get(1).unwrap().as_str().to_string());
            println!(">>time:{};", cap.get(2).unwrap().as_str().to_string());
            println!(">>pid:{};", cap.get(3).unwrap().as_str().to_string());
            let content = cap.get(4).unwrap().as_str().trim_start().to_string();
            let content = content.splitn(2, ' ').collect::<Vec<&str>>();
            println!(">>tid:{};", content[0].to_string().trim());
            let content = content[1].to_string();
            let content = content.splitn(2, '/').collect::<Vec<&str>>();
            println!("==>>level:{};", content[0].to_string());
            println!("==>>tag:{};", content[1].to_string());
        } else {
            println!("{}---->{}", s, re.is_match(s));
        }
    }
}