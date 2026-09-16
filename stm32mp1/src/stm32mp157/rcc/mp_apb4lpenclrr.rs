///Register `MP_APB4LPENCLRR` reader
pub type R = crate::R<MP_APB4LPENCLRRrs>;
///Register `MP_APB4LPENCLRR` writer
pub type W = crate::W<MP_APB4LPENCLRRrs>;
///Field `LTDCLPEN` reader - LTDCLPEN
pub type LTDCLPEN_R = crate::BitReader;
///Field `LTDCLPEN` writer - LTDCLPEN
pub type LTDCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSILPEN` reader - DSILPEN
pub type DSILPEN_R = crate::BitReader;
///Field `DSILPEN` writer - DSILPEN
pub type DSILPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRPERFMLPEN` reader - DDRPERFMLPEN
pub type DDRPERFMLPEN_R = crate::BitReader;
///Field `DDRPERFMLPEN` writer - DDRPERFMLPEN
pub type DDRPERFMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG2APBLPEN` reader - IWDG2APBLPEN
pub type IWDG2APBLPEN_R = crate::BitReader;
///Field `IWDG2APBLPEN` writer - IWDG2APBLPEN
pub type IWDG2APBLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBPHYLPEN` reader - USBPHYLPEN
pub type USBPHYLPEN_R = crate::BitReader;
///Field `USBPHYLPEN` writer - USBPHYLPEN
pub type USBPHYLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STGENROLPEN` reader - STGENROLPEN
pub type STGENROLPEN_R = crate::BitReader;
///Field `STGENROLPEN` writer - STGENROLPEN
pub type STGENROLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STGENROSTPEN` reader - STGENROSTPEN
pub type STGENROSTPEN_R = crate::BitReader;
///Field `STGENROSTPEN` writer - STGENROSTPEN
pub type STGENROSTPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LTDCLPEN
    #[inline(always)]
    pub fn ltdclpen(&self) -> LTDCLPEN_R {
        LTDCLPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - DSILPEN
    #[inline(always)]
    pub fn dsilpen(&self) -> DSILPEN_R {
        DSILPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - DDRPERFMLPEN
    #[inline(always)]
    pub fn ddrperfmlpen(&self) -> DDRPERFMLPEN_R {
        DDRPERFMLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 15 - IWDG2APBLPEN
    #[inline(always)]
    pub fn iwdg2apblpen(&self) -> IWDG2APBLPEN_R {
        IWDG2APBLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - USBPHYLPEN
    #[inline(always)]
    pub fn usbphylpen(&self) -> USBPHYLPEN_R {
        USBPHYLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - STGENROLPEN
    #[inline(always)]
    pub fn stgenrolpen(&self) -> STGENROLPEN_R {
        STGENROLPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - STGENROSTPEN
    #[inline(always)]
    pub fn stgenrostpen(&self) -> STGENROSTPEN_R {
        STGENROSTPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MP_APB4LPENCLRR")
            .field("ltdclpen", &self.ltdclpen())
            .field("dsilpen", &self.dsilpen())
            .field("ddrperfmlpen", &self.ddrperfmlpen())
            .field("iwdg2apblpen", &self.iwdg2apblpen())
            .field("usbphylpen", &self.usbphylpen())
            .field("stgenrolpen", &self.stgenrolpen())
            .field("stgenrostpen", &self.stgenrostpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - LTDCLPEN
    #[inline(always)]
    pub fn ltdclpen(&mut self) -> LTDCLPEN_W<'_, MP_APB4LPENCLRRrs> {
        LTDCLPEN_W::new(self, 0)
    }
    ///Bit 4 - DSILPEN
    #[inline(always)]
    pub fn dsilpen(&mut self) -> DSILPEN_W<'_, MP_APB4LPENCLRRrs> {
        DSILPEN_W::new(self, 4)
    }
    ///Bit 8 - DDRPERFMLPEN
    #[inline(always)]
    pub fn ddrperfmlpen(&mut self) -> DDRPERFMLPEN_W<'_, MP_APB4LPENCLRRrs> {
        DDRPERFMLPEN_W::new(self, 8)
    }
    ///Bit 15 - IWDG2APBLPEN
    #[inline(always)]
    pub fn iwdg2apblpen(&mut self) -> IWDG2APBLPEN_W<'_, MP_APB4LPENCLRRrs> {
        IWDG2APBLPEN_W::new(self, 15)
    }
    ///Bit 16 - USBPHYLPEN
    #[inline(always)]
    pub fn usbphylpen(&mut self) -> USBPHYLPEN_W<'_, MP_APB4LPENCLRRrs> {
        USBPHYLPEN_W::new(self, 16)
    }
    ///Bit 20 - STGENROLPEN
    #[inline(always)]
    pub fn stgenrolpen(&mut self) -> STGENROLPEN_W<'_, MP_APB4LPENCLRRrs> {
        STGENROLPEN_W::new(self, 20)
    }
    ///Bit 21 - STGENROSTPEN
    #[inline(always)]
    pub fn stgenrostpen(&mut self) -> STGENROSTPEN_W<'_, MP_APB4LPENCLRRrs> {
        STGENROSTPEN_W::new(self, 21)
    }
}
/**This register is used by the MCU

You can [`read`](crate::Reg::read) this register and get [`mp_apb4lpenclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb4lpenclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MP_APB4LPENCLRR)*/
pub struct MP_APB4LPENCLRRrs;
impl crate::RegisterSpec for MP_APB4LPENCLRRrs {
    type Ux = u32;
}
///`read()` method returns [`mp_apb4lpenclrr::R`](R) reader structure
impl crate::Readable for MP_APB4LPENCLRRrs {}
///`write(|w| ..)` method takes [`mp_apb4lpenclrr::W`](W) writer structure
impl crate::Writable for MP_APB4LPENCLRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MP_APB4LPENCLRR to value 0x0011_8111
impl crate::Resettable for MP_APB4LPENCLRRrs {
    const RESET_VALUE: u32 = 0x0011_8111;
}
