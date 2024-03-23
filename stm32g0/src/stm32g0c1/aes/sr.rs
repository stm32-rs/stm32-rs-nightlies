#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Field `CCF` reader - Computation completed flag This flag indicates whether the computation is completed: The flag is set by hardware upon the completion of the computation. It is cleared by software, upon setting the CCFC bit of the AES_CR register. Upon the flag setting, an interrupt is generated if enabled through the CCFIE bit of the AES_CR register. The flag is significant only when the DMAOUTEN bit is 0. It may stay high when DMA_EN is 1."]
pub type CCF_R = crate::BitReader;
#[doc = "Field `RDERR` reader - Read error flag This flag indicates the detection of an unexpected read operation from the AES_DOUTR register (during computation or data input phase): The flag is set by hardware. It is cleared by software upon setting the ERRC bit of the AES_CR register. Upon the flag setting, an interrupt is generated if enabled through the ERRIE bit of the AES_CR register. The flag setting has no impact on the AES operation. Unexpected read returns zero."]
pub type RDERR_R = crate::BitReader;
#[doc = "Field `WRERR` reader - Write error This flag indicates the detection of an unexpected write operation to the AES_DINR register (during computation or data output phase): The flag is set by hardware. It is cleared by software upon setting the ERRC bit of the AES_CR register. Upon the flag setting, an interrupt is generated if enabled through the ERRIE bit of the AES_CR register. The flag setting has no impact on the AES operation. Unexpected write is ignored."]
pub type WRERR_R = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy This flag indicates whether AES is idle or busy during GCM payload encryption phase: When the flag indicates “idle”, the current GCM encryption processing may be suspended to process a higher-priority message. In other chaining modes, or in GCM phases other than payload encryption, the flag must be ignored for the suspend process."]
pub type BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Computation completed flag This flag indicates whether the computation is completed: The flag is set by hardware upon the completion of the computation. It is cleared by software, upon setting the CCFC bit of the AES_CR register. Upon the flag setting, an interrupt is generated if enabled through the CCFIE bit of the AES_CR register. The flag is significant only when the DMAOUTEN bit is 0. It may stay high when DMA_EN is 1."]
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read error flag This flag indicates the detection of an unexpected read operation from the AES_DOUTR register (during computation or data input phase): The flag is set by hardware. It is cleared by software upon setting the ERRC bit of the AES_CR register. Upon the flag setting, an interrupt is generated if enabled through the ERRIE bit of the AES_CR register. The flag setting has no impact on the AES operation. Unexpected read returns zero."]
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write error This flag indicates the detection of an unexpected write operation to the AES_DINR register (during computation or data output phase): The flag is set by hardware. It is cleared by software upon setting the ERRC bit of the AES_CR register. Upon the flag setting, an interrupt is generated if enabled through the ERRIE bit of the AES_CR register. The flag setting has no impact on the AES operation. Unexpected write is ignored."]
    #[inline(always)]
    pub fn wrerr(&self) -> WRERR_R {
        WRERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Busy This flag indicates whether AES is idle or busy during GCM payload encryption phase: When the flag indicates “idle”, the current GCM encryption processing may be suspended to process a higher-priority message. In other chaining modes, or in GCM phases other than payload encryption, the flag must be ignored for the suspend process."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "AES status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
