extern "C" __global__ void compute_forces(
    float *positions_x, float *positions_y,
    float *forces_x, float *forces_y,
    int *neighbors, float *distances,
    int num_nodes, int max_neighbors)
{
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < num_nodes)
    {
        float pos_x = positions_x[idx];
        float pos_y = positions_y[idx];
        float force_x = 0.0f;
        float force_y = 0.0f;

        for (int i = 0; i < max_neighbors; i++)
        {
            int other_idx = neighbors[idx * max_neighbors + i];
            if (other_idx == -1)
                break; // No more neighbors

            float other_pos_x = positions_x[other_idx];
            float other_pos_y = positions_y[other_idx];
            float distance = distances[idx * max_neighbors + i];

            float delta_x = other_pos_x - pos_x;
            float delta_y = other_pos_y - pos_y;
            float dist = delta_x * delta_x + delta_y * delta_y;
            float correction = dist - (distance * distance);

            if (distance > 0.0f && dist > 1e-6f)
            {
                float scale = 0.01f * atanf(correction) / sqrtf(dist);
                force_x += delta_x * scale;
                force_y += delta_y * scale;
            }

            if (dist > 1e-6f)
            {
                float repel_scale = 0.01f / max(dist, 1.0f);
                force_x -= delta_x * repel_scale;
                force_y -= delta_y * repel_scale;
            }
        }

        forces_x[idx] = force_x;
        forces_y[idx] = force_y;
    }
}
