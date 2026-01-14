///Register `TZSC_MPCWM4BR` reader
pub type R = crate::R<TZSC_MPCWM4BRrs>;
///Register `TZSC_MPCWM4BR` writer
pub type W = crate::W<TZSC_MPCWM4BRrs>;
///Field `SUBB_START` reader - Start of sub-region B This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table 16.
pub type SUBB_START_R = crate::FieldReader<u16>;
///Field `SUBB_START` writer - Start of sub-region B This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table 16.
pub type SUBB_START_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `SUBB_LENGTH` reader - Length of sub-region B This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table 16. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled (SREN bit in GTZC1_TZSC_MPCMWBCFGR is cleared).
pub type SUBB_LENGTH_R = crate::FieldReader<u16>;
///Field `SUBB_LENGTH` writer - Length of sub-region B This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table 16. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled (SREN bit in GTZC1_TZSC_MPCMWBCFGR is cleared).
pub type SUBB_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:10 - Start of sub-region B This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table 16.
    #[inline(always)]
    pub fn subb_start(&self) -> SUBB_START_R {
        SUBB_START_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:27 - Length of sub-region B This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table 16. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled (SREN bit in GTZC1_TZSC_MPCMWBCFGR is cleared).
    #[inline(always)]
    pub fn subb_length(&self) -> SUBB_LENGTH_R {
        SUBB_LENGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZSC_MPCWM4BR")
            .field("subb_start", &self.subb_start())
            .field("subb_length", &self.subb_length())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Start of sub-region B This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table 16.
    #[inline(always)]
    pub fn subb_start(&mut self) -> SUBB_START_W<'_, TZSC_MPCWM4BRrs> {
        SUBB_START_W::new(self, 0)
    }
    ///Bits 16:27 - Length of sub-region B This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table 16. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled (SREN bit in GTZC1_TZSC_MPCMWBCFGR is cleared).
    #[inline(always)]
    pub fn subb_length(&mut self) -> SUBB_LENGTH_W<'_, TZSC_MPCWM4BRrs> {
        SUBB_LENGTH_W::new(self, 16)
    }
}
/**GTZC1 TZSC BKPSRAM sub-region B watermark register

You can [`read`](crate::Reg::read) this register and get [`tzsc_mpcwm4br::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_mpcwm4br::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#GTZC1:TZSC_MPCWM4BR)*/
pub struct TZSC_MPCWM4BRrs;
impl crate::RegisterSpec for TZSC_MPCWM4BRrs {
    type Ux = u32;
}
///`read()` method returns [`tzsc_mpcwm4br::R`](R) reader structure
impl crate::Readable for TZSC_MPCWM4BRrs {}
///`write(|w| ..)` method takes [`tzsc_mpcwm4br::W`](W) writer structure
impl crate::Writable for TZSC_MPCWM4BRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TZSC_MPCWM4BR to value 0x0800_0000
impl crate::Resettable for TZSC_MPCWM4BRrs {
    const RESET_VALUE: u32 = 0x0800_0000;
}
