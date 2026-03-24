wgpu & Graphics Fundamentals Quiz
Section 1: wgpu Components
Q1. What is the role of the Adapter, and how does it differ from the Device?

Q2. Why is it recommended to pass compatible_surface when calling request_adapter?

Q3. Per the WebGPU spec, what is the relationship between an Adapter and the number of Devices it can create?

Q4. What does DeviceDescriptor actually do — is it for querying capabilities or configuring them?

Q5. Which component do you submit command buffers to: the Device, the Queue, or the Surface?

Section 2: Surface & Window
Q6. What does SurfaceTarget represent, and why does it exist?

Q7. Why does create_surface(&window) fail to compile when window is a field on the same struct that stores the surface?

Q8. Why does Arc<Window> solve this, and what lifetime does the resulting Surface have?

Q9. Explain why a self-referential struct (one field borrowing from another) is impossible in Rust. Give two reasons.

Q10. When must you call surface.configure()? When must you call it again?

Section 3: Rust Ownership & Types
Q11. What is the difference between Move and Copy at the machine level?

Q12. Are structs stack-allocated or heap-allocated by default?

Q13. What does as_ref() do on an Option<T>, and why is it needed before .unwrap()?

Q14. In a pattern match, what is the difference between Some(x), Some(ref x), and Some(ref mut x)?

Q15. Why is ref mut the only valid binding mode inside Option::as_mut()?

Section 4: Struct Design
Q16. You have a struct with 5 Option fields that are always initialized together. What's wrong with this design, and what's the idiomatic alternative?

Q17. Should App store graphics as Option<&GraphicsContext> or Option<GraphicsContext>? Why?

Q18. What is the typestate pattern, and how does it reduce unwrap() calls?

Section 5: winit
Q19. Why is Window stored as Option in the App struct?

Q20. What is the difference between ControlFlow::Poll and ControlFlow::Wait?

Q21. Can resumed() be called more than once? Under what circumstances?

Q22. Why can't you .await inside resumed()?

Section 6: The Render Loop
Q23. Put these steps in the correct order: present, begin_render_pass, create_command_encoder, get_current_texture, queue.submit, encoder.finish, create_view.

Q24. Does the render pass borrow the encoder mutably or immutably? Why does it matter?

Q25. Does the command buffer "contain" the frame's pixel data? If not, how does the frame get populated?

Section 7: Swap Chain & Display
Q26. When are the swap chain textures allocated — at get_current_texture() or at surface.configure()?

Q27. What does present() actually do? Does it send pixels to the display?

Q28. What happens when get_current_texture() is called but all swap chain textures are still in use?

Q29. Does the display controller show the entire image at once, or sequentially? Describe the process.

Section 8: VSync & Tearing
Q30. What is a vblank signal?

Q31. Explain why tearing occurs with vsync off, using the terms "scanout" and "pointer swap."

Q32. With vsync on and a 3-texture swap chain at 60Hz, what is the approximate worst-case input-to-display latency?

Q33. Does vsync off eliminate the swap chain? Why or why not?

Q34. Why does a 3-texture swap chain make sense with vsync on but not with vsync off?

Q35. What does desired_maximum_frame_latency control? Is it the same as swap chain size?

Q36. With vsync off and a swap chain of size 2, what is the effective maximum CPU-ahead-of-GPU frame count, regardless of desired_maximum_frame_latency?

Section 9: Latency
Q37. Name three hardware-level factors that contribute to input-to-photon latency even with vsync off.

Q38. With vsync on, what is the back-pressure mechanism that prevents unbounded latency?

Q39. What is PresentMode::Mailbox and what trade-off does it make compared to Fifo and Immediate?