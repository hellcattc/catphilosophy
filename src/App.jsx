import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Post from "./Components/Post/Post.jsx";



function App() {
  const [contentFeed, setContentFeed] = useState([])
  const [loadingPosts, setLoadingPosts] = useState(0)

  const handleScroll = (event) => {
    const windowHeight = "innerHeight" in window ? window.innerHeight : document.documentElement.offsetHeight;
    const body = document.body;
    const html = document.documentElement;
    const docHeight = Math.max(body.scrollHeight, body.offsetHeight, html.clientHeight, html.scrollHeight, html.offsetHeight);
    const windowBottom = windowHeight + window.pageYOffset;
    if (windowBottom >= docHeight) {
        setLoadingPosts(loadingPosts => loadingPosts+1)
        console.log(loadingPosts)
    }
  }

  async function get_data() {
      await invoke("get_text_and_photos", {postCount: 3})
      .then((res) => {
        console.log("This are res")
        console.log(res)
        let postArray = [];
        res.map((item) => {
          postArray = [...postArray, {img: item.img_url, quote: item.quote_text}]
          console.log("changed state")
          console.log(contentFeed)
        })
        setContentFeed([...contentFeed, ...postArray])
      })
      .catch((e) => console.log(e));
    }
    

  useEffect(() => {
    document.addEventListener('scroll', handleScroll)
    get_data()
  }, [])

  useEffect(() => {
    get_data()
  }, [loadingPosts])

  const contentFeedPosts = contentFeed.map((obj, i) => {
    return <Post key = {i} imgSrc = {obj.img} quote = {obj.quote}/>
  })

  return (
    <div>
      <p>Length of photosContent: {contentFeed.length}</p>
      {contentFeedPosts}
    </div>
  );
}

export default App
