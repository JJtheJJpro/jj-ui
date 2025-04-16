import { BiWifiOff } from "react-icons/bi";

export default function Taskbar(props: { time: { hrs: number, mins: number, secs: number } }) {
    return (
        <div className="taskbar">
            <div className="quickaction">
                <div className="wifilogo"><BiWifiOff size="30px"/></div>
            </div>
            <p className="time">{props.time.hrs}:{props.time.mins.toString().padStart(2, '0')}:{props.time.secs.toString().padStart(2, '0')}<br />04/13/2025</p>
        </div>
    )
}