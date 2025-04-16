import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Taskbar from "./Components/Taskbar";
import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";

export default function App() {
    const [loaded, setLoaded] = useState(false);
    const [vTime, setTime] = useState({ hrs: 0, mins: 0, secs: 0 });

    useEffect(() => {
        if (!loaded) {
            setLoaded(true);
        } else {
            invoke('frontend_loaded');
            listen('main-clock-time-update', (e) => {
                let t = e.payload as { hrs: number, mins: number, secs: number };
                setTime({ hrs: t.hrs, mins: t.mins, secs: t.secs });
                //console.log(e.payload);
            });
        }
    }, [loaded]);

    return (
        <div className="background" style={{ backgroundImage: `url('./resources/wallpaper.jpg')`, backgroundSize: 'cover' }}>


            <Taskbar time={{ hrs: vTime.hrs, mins: vTime.mins, secs: vTime.secs }} />
        </div>
    );
}
