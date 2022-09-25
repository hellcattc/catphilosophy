import {
  useEffect, useState, useCallback, useRef, React,
} from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';
import Post from './Components/Post/Post';

function App() {
  const [contentFeed, setContentFeed] = useState([]);
  const [postsPage, setPostsPage] = useState(0);
  const loader = useRef(0);

  const handleObserver = useCallback((entries) => {
    const target = entries[0];
    if (target.isIntersecting) {
      setPostsPage((prev) => prev + 1);
    }
  }, []);

  useEffect(() => {
    const option = {
      root: null,
      rootMargin: '20px',
      threshold: 0,
    };
    const observer = new IntersectionObserver(handleObserver, option);
    if (loader.current) observer.observe(loader.current);
  }, [handleObserver]);

  useEffect(() => {
    let loading = true;

    async function getData() {
      await invoke('get_text_and_photos', { postCount: 3 })
        .then((res) => {
          const postArray = res;
          if (loading) setContentFeed((prev) => [...prev, ...postArray]);
        })
        .catch((e) => console.log(e));
    }

    getData();

    return () => { loading = false; };
  }, [postsPage]);

  const contentFeedPosts = contentFeed.map((obj, i) => (
    <Post
    // eslint-disable-next-line react/no-array-index-key
      key={i}
      imgSrc={obj.imgUrl}
      quote={obj.quoteData.quote}
      author={obj.quoteData.author ? obj.quoteData.author : undefined}
    />
  ));

  return (
    <div>
      <header>
        <p>Котофилософия</p>
        {/* <button type="button" onClick={handleExit}>Выйти</button> */}
      </header>
      <div className="div-container">
        {contentFeedPosts}
        <div ref={loader} />
      </div>
    </div>
  );
}

export default App;
