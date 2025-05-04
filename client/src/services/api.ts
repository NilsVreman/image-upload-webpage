import axios from "axios";
import AxiosMockAdapter from "axios-mock-adapter";
import type { ImageMetaData } from "@/stores/imageStore";

const api = axios.create({
  withCredentials: true,
});

if (import.meta.env.DEV) {
  const mockAdapter = new AxiosMockAdapter(api, { delayResponse: 300 });

  const mockImages: ImageMetaData[] = [
    {
      name: "img1.webp",
      uploaded_at: new Date("2025-05-01T12:00:00.000Z"),
      image_url: "/test-images/img1.webp",
      thumbnail_url: "/test-images/img1.webp",
    },
    {
      name: "img2.jpg",
      uploaded_at: new Date("2025-05-02T13:30:00.000Z"),
      image_url: "/test-images/img2.jpg",
      thumbnail_url: "/test-images/img2.jpg",
    },
    {
      name: "img3.png",
      uploaded_at: new Date("2025-05-02T14:30:00.000Z"),
      image_url: "/test-images/img3.png",
      thumbnail_url: "/test-images/img3.png",
    },
    {
      name: "img4.jpg",
      uploaded_at: new Date("2025-05-02T15:30:00.000Z"),
      image_url: "/test-images/img4.jpg",
      thumbnail_url: "/test-images/img4.jpg",
    },
    {
      name: "img5.png",
      uploaded_at: new Date("2025-05-02T16:30:00.000Z"),
      image_url: "/test-images/img5.png",
      thumbnail_url: "/test-images/img5.png",
    },
    {
      name: "img6.webp",
      uploaded_at: new Date("2025-05-03T12:00:00.000Z"),
      image_url: "/test-images/img6.webp",
      thumbnail_url: "/test-images/img6.webp",
    },
    {
      name: "img7.jpg",
      uploaded_at: new Date("2025-05-04T13:30:00.000Z"),
      image_url: "/test-images/img7.jpg",
      thumbnail_url: "/test-images/img7.jpg",
    },
    {
      name: "img8.png",
      uploaded_at: new Date("2025-05-04T14:30:00.000Z"),
      image_url: "/test-images/img8.png",
      thumbnail_url: "/test-images/img8.png",
    },
    {
      name: "img9.jpg",
      uploaded_at: new Date("2025-05-04T15:30:00.000Z"),
      image_url: "/test-images/img9.jpg",
      thumbnail_url: "/test-images/img9.jpg",
    },
    {
      name: "img10.png",
      uploaded_at: new Date("2025-05-04T16:30:00.000Z"),
      image_url: "/test-images/img10.png",
      thumbnail_url: "/test-images/img10.png",
    },
  ];

  mockAdapter.onGet("/check-session").reply(200, { valid: true });
  mockAdapter.onPost("/login").reply(config => {
    const { username, password } = JSON.parse(config.data);
    console.warn("Mock login attempt:", { username, password });
    return [200, { success: true, token: "mock-token" }];
  });

  mockAdapter.onGet("/images/thumbnails").reply(200, { images: mockImages });
  mockAdapter.onPost("/images").reply(200, { images: mockImages });
}

export default api;
