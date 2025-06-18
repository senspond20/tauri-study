import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { getCurrentWindow } from '@tauri-apps/api/window';
import { Menu } from '@tauri-apps/api/menu';

const menu = await Menu.new({
  items: [
    {
      id: 'quit',
      text: 'Quit',
      action: () => {
        console.log('quit pressed');
      },
    },
  ],
});

// If a window was not created with an explicit menu or had one set explicitly,
// this menu will be assigned to it.
menu.setAsAppMenu().then((res) => {
  console.log('menu set success', res);
});
function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const appWindows = getCurrentWindow()

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container">
      <div data-tauri-drag-region className="titlebar">
        <div className="titlebar-button" id="titlebar-minimize" onClick={()=>appWindows.minimize()}>
          <img
            src="https://api.iconify.design/mdi:window-minimize.svg"
            alt="minimize"
          />
        </div>
        <div className="titlebar-button" id="titlebar-maximize" onClick={()=>appWindows.toggleMaximize()}>
          <img
            src="https://api.iconify.design/mdi:window-maximize.svg"
            alt="maximize"
          />
        </div>
        <div className="titlebar-button" id="titlebar-close" onClick={()=>appWindows.close()}>
          <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
        </div>
      </div>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
    </main>
  );
}

export default App;
