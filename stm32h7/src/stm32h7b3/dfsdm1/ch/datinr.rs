#[doc = "Register `DATINR` reader"]
pub type R = crate::R<DATINRrs>;
#[doc = "Register `DATINR` writer"]
pub type W = crate::W<DATINRrs>;
#[doc = "Field `INDAT0` reader - Input data for channel y Input parallel channel data to be processed by the digital filter if DATMPX\\[1:0\\]=1 or DATMPX\\[1:0\\]=2. Data can be written by CPU/DMA (if DATMPX\\[1:0\\]=2) or directly by internal ADC (if DATMPX\\[1:0\\]=1). If DATPACK\\[1:0\\]=0 (standard mode) Channel y data sample is stored into INDAT0\\[15:0\\]. If DATPACK\\[1:0\\]=1 (interleaved mode) First channel y data sample is stored into INDAT0\\[15:0\\]. Second channel y data sample is stored into INDAT1\\[15:0\\]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK\\[1:0\\]=2 (dual mode). For even y channels: Channel y data sample is stored into INDAT0\\[15:0\\]. For odd y channels: INDAT0\\[15:0\\]
is write protected. See for more details. INDAT0\\[15:0\\]
is in the16-bit signed format."]
pub type INDAT0_R = crate::FieldReader<u16>;
#[doc = "Field `INDAT0` writer - Input data for channel y Input parallel channel data to be processed by the digital filter if DATMPX\\[1:0\\]=1 or DATMPX\\[1:0\\]=2. Data can be written by CPU/DMA (if DATMPX\\[1:0\\]=2) or directly by internal ADC (if DATMPX\\[1:0\\]=1). If DATPACK\\[1:0\\]=0 (standard mode) Channel y data sample is stored into INDAT0\\[15:0\\]. If DATPACK\\[1:0\\]=1 (interleaved mode) First channel y data sample is stored into INDAT0\\[15:0\\]. Second channel y data sample is stored into INDAT1\\[15:0\\]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK\\[1:0\\]=2 (dual mode). For even y channels: Channel y data sample is stored into INDAT0\\[15:0\\]. For odd y channels: INDAT0\\[15:0\\]
is write protected. See for more details. INDAT0\\[15:0\\]
is in the16-bit signed format."]
pub type INDAT0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INDAT1` reader - Input data for channel y or channel y+1 Input parallel channel data to be processed by the digital filter if DATMPX\\[1:0\\]=1 or DATMPX\\[1:0\\]=2. Data can be written by CPU/DMA (if DATMPX\\[1:0\\]=2) or directly by internal ADC (if DATMPX\\[1:0\\]=1). If DATPACK\\[1:0\\]=0 (standard mode) INDAT0\\[15:0\\]
is write protected (not used for input sample). If DATPACK\\[1:0\\]=1 (interleaved mode) Second channel y data sample is stored into INDAT1\\[15:0\\]. First channel y data sample is stored into INDAT0\\[15:0\\]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK\\[1:0\\]=2 (dual mode). For even y channels: sample in INDAT1\\[15:0\\]
is automatically copied into INDAT0\\[15:0\\]
of channel (y+1). For odd y channels: INDAT1\\[15:0\\]
is write protected. See for more details. INDAT0\\[15:1\\]
is in the16-bit signed format."]
pub type INDAT1_R = crate::FieldReader<u16>;
#[doc = "Field `INDAT1` writer - Input data for channel y or channel y+1 Input parallel channel data to be processed by the digital filter if DATMPX\\[1:0\\]=1 or DATMPX\\[1:0\\]=2. Data can be written by CPU/DMA (if DATMPX\\[1:0\\]=2) or directly by internal ADC (if DATMPX\\[1:0\\]=1). If DATPACK\\[1:0\\]=0 (standard mode) INDAT0\\[15:0\\]
is write protected (not used for input sample). If DATPACK\\[1:0\\]=1 (interleaved mode) Second channel y data sample is stored into INDAT1\\[15:0\\]. First channel y data sample is stored into INDAT0\\[15:0\\]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK\\[1:0\\]=2 (dual mode). For even y channels: sample in INDAT1\\[15:0\\]
is automatically copied into INDAT0\\[15:0\\]
of channel (y+1). For odd y channels: INDAT1\\[15:0\\]
is write protected. See for more details. INDAT0\\[15:1\\]
is in the16-bit signed format."]
pub type INDAT1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data for channel y Input parallel channel data to be processed by the digital filter if DATMPX\\[1:0\\]=1 or DATMPX\\[1:0\\]=2. Data can be written by CPU/DMA (if DATMPX\\[1:0\\]=2) or directly by internal ADC (if DATMPX\\[1:0\\]=1). If DATPACK\\[1:0\\]=0 (standard mode) Channel y data sample is stored into INDAT0\\[15:0\\]. If DATPACK\\[1:0\\]=1 (interleaved mode) First channel y data sample is stored into INDAT0\\[15:0\\]. Second channel y data sample is stored into INDAT1\\[15:0\\]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK\\[1:0\\]=2 (dual mode). For even y channels: Channel y data sample is stored into INDAT0\\[15:0\\]. For odd y channels: INDAT0\\[15:0\\]
is write protected. See for more details. INDAT0\\[15:0\\]
is in the16-bit signed format."]
    #[inline(always)]
    pub fn indat0(&self) -> INDAT0_R {
        INDAT0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Input data for channel y or channel y+1 Input parallel channel data to be processed by the digital filter if DATMPX\\[1:0\\]=1 or DATMPX\\[1:0\\]=2. Data can be written by CPU/DMA (if DATMPX\\[1:0\\]=2) or directly by internal ADC (if DATMPX\\[1:0\\]=1). If DATPACK\\[1:0\\]=0 (standard mode) INDAT0\\[15:0\\]
is write protected (not used for input sample). If DATPACK\\[1:0\\]=1 (interleaved mode) Second channel y data sample is stored into INDAT1\\[15:0\\]. First channel y data sample is stored into INDAT0\\[15:0\\]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK\\[1:0\\]=2 (dual mode). For even y channels: sample in INDAT1\\[15:0\\]
is automatically copied into INDAT0\\[15:0\\]
of channel (y+1). For odd y channels: INDAT1\\[15:0\\]
is write protected. See for more details. INDAT0\\[15:1\\]
is in the16-bit signed format."]
    #[inline(always)]
    pub fn indat1(&self) -> INDAT1_R {
        INDAT1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Input data for channel y Input parallel channel data to be processed by the digital filter if DATMPX\\[1:0\\]=1 or DATMPX\\[1:0\\]=2. Data can be written by CPU/DMA (if DATMPX\\[1:0\\]=2) or directly by internal ADC (if DATMPX\\[1:0\\]=1). If DATPACK\\[1:0\\]=0 (standard mode) Channel y data sample is stored into INDAT0\\[15:0\\]. If DATPACK\\[1:0\\]=1 (interleaved mode) First channel y data sample is stored into INDAT0\\[15:0\\]. Second channel y data sample is stored into INDAT1\\[15:0\\]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK\\[1:0\\]=2 (dual mode). For even y channels: Channel y data sample is stored into INDAT0\\[15:0\\]. For odd y channels: INDAT0\\[15:0\\]
is write protected. See for more details. INDAT0\\[15:0\\]
is in the16-bit signed format."]
    #[inline(always)]
    #[must_use]
    pub fn indat0(&mut self) -> INDAT0_W<DATINRrs> {
        INDAT0_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Input data for channel y or channel y+1 Input parallel channel data to be processed by the digital filter if DATMPX\\[1:0\\]=1 or DATMPX\\[1:0\\]=2. Data can be written by CPU/DMA (if DATMPX\\[1:0\\]=2) or directly by internal ADC (if DATMPX\\[1:0\\]=1). If DATPACK\\[1:0\\]=0 (standard mode) INDAT0\\[15:0\\]
is write protected (not used for input sample). If DATPACK\\[1:0\\]=1 (interleaved mode) Second channel y data sample is stored into INDAT1\\[15:0\\]. First channel y data sample is stored into INDAT0\\[15:0\\]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK\\[1:0\\]=2 (dual mode). For even y channels: sample in INDAT1\\[15:0\\]
is automatically copied into INDAT0\\[15:0\\]
of channel (y+1). For odd y channels: INDAT1\\[15:0\\]
is write protected. See for more details. INDAT0\\[15:1\\]
is in the16-bit signed format."]
    #[inline(always)]
    #[must_use]
    pub fn indat1(&mut self) -> INDAT1_W<DATINRrs> {
        INDAT1_W::new(self, 16)
    }
}
#[doc = "DFSDM channel 0 data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datinr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datinr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATINRrs;
impl crate::RegisterSpec for DATINRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datinr::R`](R) reader structure"]
impl crate::Readable for DATINRrs {}
#[doc = "`write(|w| ..)` method takes [`datinr::W`](W) writer structure"]
impl crate::Writable for DATINRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATINR to value 0"]
impl crate::Resettable for DATINRrs {
    const RESET_VALUE: u32 = 0;
}
