///Register `MC_APB4LPENSETR` reader
pub type R = crate::R<MC_APB4LPENSETRrs>;
///Register `MC_APB4LPENSETR` writer
pub type W = crate::W<MC_APB4LPENSETRrs>;
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
        f.debug_struct("MC_APB4LPENSETR")
            .field("ltdclpen", &self.ltdclpen())
            .field("dsilpen", &self.dsilpen())
            .field("ddrperfmlpen", &self.ddrperfmlpen())
            .field("usbphylpen", &self.usbphylpen())
            .field("stgenrolpen", &self.stgenrolpen())
            .field("stgenrostpen", &self.stgenrostpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - LTDCLPEN
    #[inline(always)]
    pub fn ltdclpen(&mut self) -> LTDCLPEN_W<'_, MC_APB4LPENSETRrs> {
        LTDCLPEN_W::new(self, 0)
    }
    ///Bit 4 - DSILPEN
    #[inline(always)]
    pub fn dsilpen(&mut self) -> DSILPEN_W<'_, MC_APB4LPENSETRrs> {
        DSILPEN_W::new(self, 4)
    }
    ///Bit 8 - DDRPERFMLPEN
    #[inline(always)]
    pub fn ddrperfmlpen(&mut self) -> DDRPERFMLPEN_W<'_, MC_APB4LPENSETRrs> {
        DDRPERFMLPEN_W::new(self, 8)
    }
    ///Bit 16 - USBPHYLPEN
    #[inline(always)]
    pub fn usbphylpen(&mut self) -> USBPHYLPEN_W<'_, MC_APB4LPENSETRrs> {
        USBPHYLPEN_W::new(self, 16)
    }
    ///Bit 20 - STGENROLPEN
    #[inline(always)]
    pub fn stgenrolpen(&mut self) -> STGENROLPEN_W<'_, MC_APB4LPENSETRrs> {
        STGENROLPEN_W::new(self, 20)
    }
    ///Bit 21 - STGENROSTPEN
    #[inline(always)]
    pub fn stgenrostpen(&mut self) -> STGENROSTPEN_W<'_, MC_APB4LPENSETRrs> {
        STGENROSTPEN_W::new(self, 21)
    }
}
/**This register is used by the MCU in order to set the PERxLPEN bit.

You can [`read`](crate::Reg::read) this register and get [`mc_apb4lpensetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb4lpensetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MC_APB4LPENSETR)*/
pub struct MC_APB4LPENSETRrs;
impl crate::RegisterSpec for MC_APB4LPENSETRrs {
    type Ux = u32;
}
///`read()` method returns [`mc_apb4lpensetr::R`](R) reader structure
impl crate::Readable for MC_APB4LPENSETRrs {}
///`write(|w| ..)` method takes [`mc_apb4lpensetr::W`](W) writer structure
impl crate::Writable for MC_APB4LPENSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MC_APB4LPENSETR to value 0x0011_0111
impl crate::Resettable for MC_APB4LPENSETRrs {
    const RESET_VALUE: u32 = 0x0011_0111;
}
