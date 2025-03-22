import "./App.css";
import Taskbar from "./Components/Taskbar";

export default function App() {
    return (
        <div className="background" style={{ backgroundImage: `url('./resources/wallpaper.jpg')`, backgroundSize: 'cover' }}>


            <Taskbar />
        </div>
    );
}
