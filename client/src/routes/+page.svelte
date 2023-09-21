<div class="home">
    <div class="content-wrapper">
        <h3 class="home-title">hallovon的小站</h3>
        <p class="home-slogan">欢迎来到我的博客</p>
    </div>
</div>

<style>
    .home {
        height: 100vh;
        width: 100%;
        background-image: url("home.jpg");
        background-size: cover;
        z-index: -100;

        position: relative;
    }

    .content-wrapper {
        color: #f1f3f5;
        position: absolute;
        top: 35%;
        left: 50%;
        transform: translateX(-50%);

        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .home-title {
        font-size: 5.4rem;
    }
    .home-slogan {
        font-size: 2rem;
    }
</style>
