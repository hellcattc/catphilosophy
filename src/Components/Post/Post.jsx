import React from 'react';
import './Post.css';
import MyImg from './MyImg';

// eslint-disable-next-line react/prop-types
function Post({ imgSrc, quote, author }) {
  return (
    <div className="post-container">
      <div className="inner-container">
        <p>{quote}</p>
        <p style={{ textAlign: 'right' }}>{author}</p>
        <MyImg src={imgSrc} />
      </div>
    </div>
  );
}

export default Post;
