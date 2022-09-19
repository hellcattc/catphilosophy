import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Post from "./Components/Post/Post.jsx";

function App() {
  const [photosContent, setPhotosContent] = useState([]);

  useEffect(() => {
    async function get_data() {
      await invoke("get_cat_photo").then((res) => setPhotosContent([...photosContent, res])).catch((e) => console.log(e))
    }
    
    if (photosContent.length < 3) {
      get_data()  
    } 
  })

  let contentFeed = photosContent.map((imgUrl, i) => {
    return <Post key = {i} imgSrc = {imgUrl}/>
  })

  return (
    <div>
      <p>Length of photosContent: {photosContent.length}</p>
      {contentFeed}
    </div>
  );
}

export default App
