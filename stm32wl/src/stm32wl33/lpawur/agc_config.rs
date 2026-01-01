///Register `AGC_CONFIG` reader
pub type R = crate::R<AGC_CONFIGrs>;
///Register `AGC_CONFIG` writer
pub type W = crate::W<AGC_CONFIGrs>;
///Field `AGC_MODE` reader - Define the working mode of the AGC:
pub type AGC_MODE_R = crate::FieldReader;
///Field `AGC_MODE` writer - Define the working mode of the AGC:
pub type AGC_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `AGC_HOLD_MODE` reader - The behavior when the AGC is ON and is working in HOLD mode
pub type AGC_HOLD_MODE_R = crate::BitReader;
///Field `AGC_HOLD_MODE` writer - The behavior when the AGC is ON and is working in HOLD mode
pub type AGC_HOLD_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AGC_RESET_MODE` reader - The AGC reset behavior when the AGC is working in ON or HOLD mode
pub type AGC_RESET_MODE_R = crate::BitReader;
///Field `AGC_RESET_MODE` writer - The AGC reset behavior when the AGC is working in ON or HOLD mode
pub type AGC_RESET_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Define the working mode of the AGC:
    #[inline(always)]
    pub fn agc_mode(&self) -> AGC_MODE_R {
        AGC_MODE_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - The behavior when the AGC is ON and is working in HOLD mode
    #[inline(always)]
    pub fn agc_hold_mode(&self) -> AGC_HOLD_MODE_R {
        AGC_HOLD_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - The AGC reset behavior when the AGC is working in ON or HOLD mode
    #[inline(always)]
    pub fn agc_reset_mode(&self) -> AGC_RESET_MODE_R {
        AGC_RESET_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGC_CONFIG")
            .field("agc_mode", &self.agc_mode())
            .field("agc_hold_mode", &self.agc_hold_mode())
            .field("agc_reset_mode", &self.agc_reset_mode())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Define the working mode of the AGC:
    #[inline(always)]
    pub fn agc_mode(&mut self) -> AGC_MODE_W<'_, AGC_CONFIGrs> {
        AGC_MODE_W::new(self, 0)
    }
    ///Bit 2 - The behavior when the AGC is ON and is working in HOLD mode
    #[inline(always)]
    pub fn agc_hold_mode(&mut self) -> AGC_HOLD_MODE_W<'_, AGC_CONFIGrs> {
        AGC_HOLD_MODE_W::new(self, 2)
    }
    ///Bit 3 - The AGC reset behavior when the AGC is working in ON or HOLD mode
    #[inline(always)]
    pub fn agc_reset_mode(&mut self) -> AGC_RESET_MODE_W<'_, AGC_CONFIGrs> {
        AGC_RESET_MODE_W::new(self, 3)
    }
}
/**AGC_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`agc_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LPAWUR:AGC_CONFIG)*/
pub struct AGC_CONFIGrs;
impl crate::RegisterSpec for AGC_CONFIGrs {
    type Ux = u32;
}
///`read()` method returns [`agc_config::R`](R) reader structure
impl crate::Readable for AGC_CONFIGrs {}
///`write(|w| ..)` method takes [`agc_config::W`](W) writer structure
impl crate::Writable for AGC_CONFIGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGC_CONFIG to value 0
impl crate::Resettable for AGC_CONFIGrs {}
