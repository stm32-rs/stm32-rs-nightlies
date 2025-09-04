///Register `TZSC_MPCWM4AR` reader
pub type R = crate::R<TZSC_MPCWM4ARrs>;
///Register `TZSC_MPCWM4AR` writer
pub type W = crate::W<TZSC_MPCWM4ARrs>;
///Field `SUBA_START` reader - Start of sub-region A This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table 16.
pub type SUBA_START_R = crate::FieldReader<u16>;
///Field `SUBA_START` writer - Start of sub-region A This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table 16.
pub type SUBA_START_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `SUBA_LENGTH` reader - Length of sub-region A This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table 16. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled (SREN bit in GTZC1_TZSC_MPCMWACFGR is cleared).
pub type SUBA_LENGTH_R = crate::FieldReader<u16>;
///Field `SUBA_LENGTH` writer - Length of sub-region A This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table 16. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled (SREN bit in GTZC1_TZSC_MPCMWACFGR is cleared).
pub type SUBA_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:10 - Start of sub-region A This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table 16.
    #[inline(always)]
    pub fn suba_start(&self) -> SUBA_START_R {
        SUBA_START_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:27 - Length of sub-region A This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table 16. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled (SREN bit in GTZC1_TZSC_MPCMWACFGR is cleared).
    #[inline(always)]
    pub fn suba_length(&self) -> SUBA_LENGTH_R {
        SUBA_LENGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZSC_MPCWM4AR")
            .field("suba_start", &self.suba_start())
            .field("suba_length", &self.suba_length())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Start of sub-region A This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table 16.
    #[inline(always)]
    pub fn suba_start(&mut self) -> SUBA_START_W<TZSC_MPCWM4ARrs> {
        SUBA_START_W::new(self, 0)
    }
    ///Bits 16:27 - Length of sub-region A This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table 16. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled (SREN bit in GTZC1_TZSC_MPCMWACFGR is cleared).
    #[inline(always)]
    pub fn suba_length(&mut self) -> SUBA_LENGTH_W<TZSC_MPCWM4ARrs> {
        SUBA_LENGTH_W::new(self, 16)
    }
}
/**GTZC1 TZSC BKPSRAM sub-region A watermark register

You can [`read`](crate::Reg::read) this register and get [`tzsc_mpcwm4ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_mpcwm4ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#GTZC1:TZSC_MPCWM4AR)*/
pub struct TZSC_MPCWM4ARrs;
impl crate::RegisterSpec for TZSC_MPCWM4ARrs {
    type Ux = u32;
}
///`read()` method returns [`tzsc_mpcwm4ar::R`](R) reader structure
impl crate::Readable for TZSC_MPCWM4ARrs {}
///`write(|w| ..)` method takes [`tzsc_mpcwm4ar::W`](W) writer structure
impl crate::Writable for TZSC_MPCWM4ARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TZSC_MPCWM4AR to value 0x0800_0000
impl crate::Resettable for TZSC_MPCWM4ARrs {
    const RESET_VALUE: u32 = 0x0800_0000;
}
