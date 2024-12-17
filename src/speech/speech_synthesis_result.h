#pragma once
#include <string>
#include <chrono>
#include <speechapi_cxx_common.h>
#include <speechapi_cxx_string_helpers.h>
#include <speechapi_cxx_enums.h>
#include <speechapi_cxx_properties.h>
#include <speechapi_cxx_audio_data_stream.h>
#include <speechapi_c_result.h>
#include <speechapi_c_synthesizer.h>

namespace Microsoft {
namespace CognitiveServices {
namespace Speech {

/// <summary>
/// Contains information about result from text-to-speech synthesis.
/// Added in version 1.4.0
/// </summary>
class SpeechSynthesisResult
{
private:

    /// <summary>
    /// Internal member variable that holds the tts result handle.
    /// </summary>
    SPXRESULTHANDLE m_hresult;

    /*! \cond PRIVATE */

    class PrivatePropertyCollection : public PropertyCollection
    {
    public:
        PrivatePropertyCollection(SPXRESULTHANDLE hresult) :
            PropertyCollection(
                [=]() {
            SPXPROPERTYBAGHANDLE hpropbag = SPXHANDLE_INVALID;
            synth_result_get_property_bag(hresult, &hpropbag);
            return hpropbag;
        }())
        {
        }
    };

    /// <summary>
    /// Internal member variable that holds the properties associating to the tts result.
    /// </summary>
    PrivatePropertyCollection m_properties;

    /*! \endcond */

public:

    /// <summary>
    /// Creates a new instance using the provided handle.
    /// </summary>
    /// <param name="hresult">Result handle.</param>
    explicit SpeechSynthesisResult(SPXRESULTHANDLE hresult) :
        m_hresult(hresult),
        m_properties(hresult),
        ResultId(m_resultId),
        Reason(m_reason),
        AudioDuration(m_audioDuration),
        Properties(m_properties)
    {
        SPX_DBG_TRACE_SCOPE(__FUNCTION__, __FUNCTION__);

        const size_t maxCharCount = 1024;
        char sz[maxCharCount + 1];

        SPX_THROW_ON_FAIL(synth_result_get_result_id(hresult, sz, maxCharCount));
        m_resultId = Utils::ToSPXString(sz);

        Result_Reason resultReason;
        SPX_THROW_ON_FAIL(synth_result_get_reason(hresult, &resultReason));
        m_reason = static_cast<ResultReason>(resultReason);

        uint32_t audioLength = 0;
        uint64_t audioDuration = 0;
        SPX_THROW_ON_FAIL(synth_result_get_audio_length_duration(m_hresult, &audioLength, &audioDuration));
        m_audioDuration = std::chrono::milliseconds(audioDuration);

        m_audioData = std::make_shared<std::vector<uint8_t>>(audioLength);

        if (audioLength > 0)
        {
            uint32_t filledSize = 0;
            SPX_THROW_ON_FAIL(synth_result_get_audio_data(m_hresult, m_audioData->data(), audioLength, &filledSize));
        }
    }

    /// <summary>
    /// Gets the size of synthesized audio in bytes.
    /// </summary>
    /// <returns>Length of synthesized audio</returns>
    uint32_t GetAudioLength()
    {
        return static_cast<uint32_t>(m_audioData->size());
    }

    /// <summary>
    /// Gets the synthesized audio.
    /// </summary>
    /// <returns>Synthesized audio data</returns>
    std::shared_ptr<std::vector<uint8_t>> GetAudioData()
    {
        return m_audioData;
    }

    /// <summary>
    /// Explicit conversion operator.
    /// </summary>
    /// <returns>A handle.</returns>
    explicit operator SPXRESULTHANDLE() { return m_hresult; }

    /// <summary>
    /// Destructor.
    /// </summary>
    ~SpeechSynthesisResult()
    {
        SPX_DBG_TRACE_SCOPE(__FUNCTION__, __FUNCTION__);
        synthesizer_result_handle_release(m_hresult);
    }

    /// <summary>
    /// Unique result id.
    /// </summary>
    const SPXSTRING& ResultId;

    /// <summary>
    /// Reason of the synthesis result.
    /// </summary>
    const ResultReason& Reason;

    /// <summary>
    /// Time duration of the synthesized audio, only valid for completed synthsis.
    /// Added in version 1.21.0
    /// </summary>
    const std::chrono::milliseconds& AudioDuration;

    /// <summary>
    /// Collection of additional SpeechSynthesisResult properties.
    /// </summary>
    const PropertyCollection& Properties;

private:

    DISABLE_DEFAULT_CTORS(SpeechSynthesisResult);

    /// <summary>
    /// Internal member variable that holds the result ID.
    /// </summary>
    SPXSTRING m_resultId;

    /// <summary>
    /// Internal member variable that holds the result reason.
    /// </summary>
    ResultReason m_reason;

    /// <summary>
    /// Internal member variable that holds the audio data
    /// </summary>
    std::shared_ptr<std::vector<uint8_t>> m_audioData;

    /// <summary>
    /// Internal member variable that holds the audio duration
    // </summary>
    std::chrono::milliseconds m_audioDuration;
};
} } } // Microsoft::CognitiveServices::Speech