// The MIT License (MIT)
//
// Copyright (c) 2013 Jeremy Letang (letang.jeremy@gmail.com)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! The functionnality that a Sound or a Music should provide.

use states::State;

/// The functionnality that an Audio Source should provide.
pub trait AudioController {
    /**
     * Play or resume the Audio Source.
     */
    fn play(&mut self) -> ();

    /**
     * Pause the Audio Source.
     */
    fn pause(&mut self) -> ();

    /**
     * Stop the Audio Source.
     */
    fn stop(&mut self) -> ();

    /**
     * Check if the Audio Source is playing or not.
     *
     * # Return
     * true if the Audio Source is playing, false otherwise.
     */
    fn is_playing(&self) -> bool;

    /**
     * Get the current state of the Audio Source
     *
     * # Return
     * The state of the Audio Source as a variant of the enum State
     */
    fn get_state(&self) -> State;

    /**
     * Set the volume of the Audio Source.
     *
     * A value of 1.0 means unattenuated. Each division by 2 equals an attenuation
     * of about -6dB. Each multiplicaton by 2 equals an amplification of about
     * +6dB.
     *
     * # Argument
     * * `volume` - The volume of the Audio Source, should be between 0. and 1. 
     */
    fn set_volume(&mut self, volume : f32) -> ();

    /**
     * Get the volume of the Audio Source.
     *
     * # Return
     * The volume of the Audio Source between 0. and 1.
     */
    fn get_volume(&self) -> f32;

    /**
     * Set the minimal volume for a Audio Source.
     *
     * The minimum volume allowed for a source, after distance and cone
     * attenation is applied (if applicable).
     *
     * # Argument
     * * `min_volume` - The new minimal volume of the Audio Source should be
     * between 0. and 1.
     */
    fn set_min_volume(&mut self, min_volume : f32) -> ();

    /**
     * Get the minimal volume of the Audio Source.
     *
     * # Return
     * The minimal volume of the Audio Source between 0. and 1.
     */
    fn get_min_volume(&self) -> f32;

    /**
     * Set the maximal volume for a Audio Source.
     *
     * The maximum volume allowed for a sound, after distance and cone
     * attenation is applied (if applicable).
     *
     * # Argument
     * * `max_volume` - The new maximal volume of the Audio Source should be
     * between 0. and 1.
     */
    fn set_max_volume(&mut self, max_volume : f32) -> ();

    /**
     * Get the maximal volume of the Audio Source.
     *
     * # Return
     * The maximal volume of the Audio Source between 0. and 1.
     */
    fn get_max_volume(&self) -> f32;

    /**
     * Set the Audio Source looping or not
     *
     * The default looping is false.
     *
     * # Arguments
     * `looping` - The new looping state.
     */
    fn set_looping(&mut self, looping : bool) -> ();

    /**
     * Check if the Audio Source is looping or not
     *
     * # Return
     * True if the Audio Source is looping, false otherwise.
     */
    fn is_looping(&self) -> bool;

    /**
     * Set the pitch of the source.
     *
     * A multiplier for the frequency (sample rate) of the source's buffer.
     *
     * Default pitch is 1.0.
     *
     * # Argument
     * * `new_pitch` - The new pitch of the Audio Source in the range
     * [0.5 - 2.0]
     */
    fn set_pitch(&mut self, pitch : f32) -> ();

    /**
     * Set the pitch of the source.
     *
     * # Return
     * The pitch of the Audio Source in the range [0.5 - 2.0]
     */
    fn get_pitch(&self) -> f32;

    /**
     * Set the position of the Audio Source relative to the listener or absolute.
     *
     * Default position is absolute.
     *
     * # Argument
     * `relative` - True to set Audio Source relative to the
     * listener false to set the Audio Source position absolute.
     */
    fn set_relative(&mut self, relative : bool) -> ();

    /**
     * Is the Audio Source relative to the listener or not ?
     *
     * # Return
     * True if the Audio Source is relative to the listener false otherwise
     */
    fn is_relative(&mut self) -> bool;

    /**
     * Set the Audio Source location in three dimensional space.
     *
     * OpenAL, like OpenGL, uses a right handed coordinate system, where in a
     * frontal default view X (thumb) points right, Y points up (index finger),
     * and Z points towards the viewer/camera (middle finger).
     * To switch from a left handed coordinate system, flip the sign on the Z
     * coordinate.
     *
     * Default position is [0., 0., 0.].
     *
     * # Argument
     * * `position` - A three dimensional vector of f32 containing the
     * position of the listener [x, y, z].
     */
    fn set_position(&mut self, position : [f32, ..3]) -> ();

    /**
     * Get the position of the Audio Source in three dimensional space.
     *
     * # Return
     * A three dimensional vector of f32 containing the position of the
     * listener [x, y, z].
     */
    fn get_position(&self) -> [f32, ..3];

    /**
     * Set the direction of the Audio Source.
     *
     * Specifies the current direction in local space.
     *
     * The default direction is: [0., 0., 0.]
     *
     * # Argument
     * `direction` - The new direction of the Audio Source.
     */
    fn set_direction(&mut self, direction : [f32, ..3]) -> ();

    /**
     * Get the direction of the Audio Source.
     *
     * # Return
     * The current direction of the Audio Source.
     */
    fn get_direction(&self)  -> [f32, ..3];

    /**
     * Set the maximum distance of the Audio Source.
     *
     * The distance above which the source is not attenuated any further with a
     * clamped distance model, or where attenuation reaches 0.0 gain for linear
     * distance models with a default rolloff factor.
     *
     * The default maximum distance is +inf.
     *
     * # Argument
     * `max_distance` - The new maximum distance in the range [0., +inf]
     */
    fn set_max_distance(&mut self, max_distance : f32) -> ();

    /**
     * Get the maximum distance of the Audio Source.
     *
     * # Return
     * The maximum distance of the Audio Source in the range [0., +inf]
     */
    fn get_max_distance(&self) -> f32;

    /**
     * Set the reference distance of the Audio Source.
     *
     * The distance in units that no attenuation occurs.
     * At 0.0, no distance attenuation ever occurs on non-linear
     * attenuation models.
     *
     * The default distance reference is 1.
     *
     * # Argument
     * * `ref_distance` - The new reference distance of the Audio Source.
     */
    fn set_reference_distance(&mut self, ref_distance : f32) -> ();

    /**
     * Get the reference distance of the Audio Source.
     *
     * # Return
     * The current reference distance of the Audio Source.
     */
    fn get_reference_distance(&self) -> f32;

    /**
     * Set the attenuation of a Audio Source.
     *
     * Multiplier to exaggerate or diminish distance attenuation.
     * At 0.0, no distance attenuation ever occurs.
     *
     * The default attenuation is 1.
     *
     * # Arguments
     * `attenuation` - The new attenuation for the Audio Source in the
     * range [0., 1.].
     */
    fn set_attenuation(&mut self, attenuation : f32) -> ();

    /**
     * Get the attenuation of a Sound.
     *
     * # Return
     * The current attenuation for the Audio Source in the range [0., 1.].
     */
    fn get_attenuation(&self) -> f32;
}

