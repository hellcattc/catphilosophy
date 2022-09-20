import { useEffect, useState, useCallback, useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Post from "./Components/Post/Post.jsx";

function App() {
  const [contentFeed, setContentFeed] = useState([])
  const [postsPage, setPostsPage] = useState(0)
  const loader = useRef(0)

  const handleObserver = useCallback((entries) => {
    const target = entries[0];
    if (target.isIntersecting) {
      setPostsPage((prev) => prev + 1);
    }
  }, []);

  useEffect(() => {
    const option = {
      root: null,
      rootMargin: "20px",
      threshold: 0
    };
    const observer = new IntersectionObserver(handleObserver, option);
    if (loader.current) observer.observe(loader.current);
  }, [handleObserver]);

  async function get_data() {
      await invoke("get_text_and_photos", {postCount: 3})
      .then((res) => {
        console.log("This are res")
        console.log(res)
        let postArray = [];
        res.map((item) => {
          postArray = [...postArray, {img: item.img_url, quote: item.quote_text}]
        })
        setContentFeed([...contentFeed, ...postArray])
        console.log("Updated state array")
        console.log(contentFeed)
      })
      .catch((e) => console.log(e));
    }
    

  useEffect(() => {
    get_data()
  }, [])

  useEffect(() => {
    get_data()
  }, [postsPage])

  const contentFeedPosts = contentFeed.map((obj, i) => {
    return <Post key = {i} imgSrc = {obj.img} quote = {obj.quote}/>
  })

  return (
    <div>
      <p>Length of photosContent: {contentFeed.length}</p>
      {contentFeedPosts}
      <div ref = {loader}></div>
    </div>
  );
}

export default App
