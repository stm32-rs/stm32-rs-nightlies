///Register `PLL` reader
pub type R = crate::R<PLLrs>;
///Register `PLL` writer
pub type W = crate::W<PLLrs>;
///Field `PLLNDIV` reader - PLLNDIV
pub type PLLNDIV_R = crate::FieldReader;
///Field `PLLNDIV` writer - PLLNDIV
pub type PLLNDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `PLLODF` reader - PLLODF
pub type PLLODF_R = crate::FieldReader;
///Field `PLLODF` writer - PLLODF
pub type PLLODF_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PLLFRACIN` reader - PLLFRACIN
pub type PLLFRACIN_R = crate::FieldReader<u16>;
///Field `PLLFRACIN` writer - PLLFRACIN
pub type PLLFRACIN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `PLLEN` reader - PLLEN
pub type PLLEN_R = crate::BitReader;
///Field `PLLEN` writer - PLLEN
pub type PLLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLSTRB` reader - PLLSTRB
pub type PLLSTRB_R = crate::BitReader;
///Field `PLLSTRB` writer - PLLSTRB
pub type PLLSTRB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLSTRBYP` reader - PLLSTRBYP
pub type PLLSTRBYP_R = crate::BitReader;
///Field `PLLSTRBYP` writer - PLLSTRBYP
pub type PLLSTRBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLFRACCTL` reader - PLLFRACCTL
pub type PLLFRACCTL_R = crate::BitReader;
///Field `PLLFRACCTL` writer - PLLFRACCTL
pub type PLLFRACCTL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLDITHEN0` reader - PLLDITHEN0
pub type PLLDITHEN0_R = crate::BitReader;
///Field `PLLDITHEN0` writer - PLLDITHEN0
pub type PLLDITHEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLDITHEN1` reader - PLLDITHEN1
pub type PLLDITHEN1_R = crate::BitReader;
///Field `PLLDITHEN1` writer - PLLDITHEN1
pub type PLLDITHEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:6 - PLLNDIV
    #[inline(always)]
    pub fn pllndiv(&self) -> PLLNDIV_R {
        PLLNDIV_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 7:9 - PLLODF
    #[inline(always)]
    pub fn pllodf(&self) -> PLLODF_R {
        PLLODF_R::new(((self.bits >> 7) & 7) as u8)
    }
    ///Bits 10:25 - PLLFRACIN
    #[inline(always)]
    pub fn pllfracin(&self) -> PLLFRACIN_R {
        PLLFRACIN_R::new(((self.bits >> 10) & 0xffff) as u16)
    }
    ///Bit 26 - PLLEN
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - PLLSTRB
    #[inline(always)]
    pub fn pllstrb(&self) -> PLLSTRB_R {
        PLLSTRB_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - PLLSTRBYP
    #[inline(always)]
    pub fn pllstrbyp(&self) -> PLLSTRBYP_R {
        PLLSTRBYP_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - PLLFRACCTL
    #[inline(always)]
    pub fn pllfracctl(&self) -> PLLFRACCTL_R {
        PLLFRACCTL_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - PLLDITHEN0
    #[inline(always)]
    pub fn plldithen0(&self) -> PLLDITHEN0_R {
        PLLDITHEN0_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - PLLDITHEN1
    #[inline(always)]
    pub fn plldithen1(&self) -> PLLDITHEN1_R {
        PLLDITHEN1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL")
            .field("pllndiv", &self.pllndiv())
            .field("pllodf", &self.pllodf())
            .field("pllfracin", &self.pllfracin())
            .field("pllen", &self.pllen())
            .field("pllstrb", &self.pllstrb())
            .field("pllstrbyp", &self.pllstrbyp())
            .field("pllfracctl", &self.pllfracctl())
            .field("plldithen0", &self.plldithen0())
            .field("plldithen1", &self.plldithen1())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - PLLNDIV
    #[inline(always)]
    pub fn pllndiv(&mut self) -> PLLNDIV_W<'_, PLLrs> {
        PLLNDIV_W::new(self, 0)
    }
    ///Bits 7:9 - PLLODF
    #[inline(always)]
    pub fn pllodf(&mut self) -> PLLODF_W<'_, PLLrs> {
        PLLODF_W::new(self, 7)
    }
    ///Bits 10:25 - PLLFRACIN
    #[inline(always)]
    pub fn pllfracin(&mut self) -> PLLFRACIN_W<'_, PLLrs> {
        PLLFRACIN_W::new(self, 10)
    }
    ///Bit 26 - PLLEN
    #[inline(always)]
    pub fn pllen(&mut self) -> PLLEN_W<'_, PLLrs> {
        PLLEN_W::new(self, 26)
    }
    ///Bit 27 - PLLSTRB
    #[inline(always)]
    pub fn pllstrb(&mut self) -> PLLSTRB_W<'_, PLLrs> {
        PLLSTRB_W::new(self, 27)
    }
    ///Bit 28 - PLLSTRBYP
    #[inline(always)]
    pub fn pllstrbyp(&mut self) -> PLLSTRBYP_W<'_, PLLrs> {
        PLLSTRBYP_W::new(self, 28)
    }
    ///Bit 29 - PLLFRACCTL
    #[inline(always)]
    pub fn pllfracctl(&mut self) -> PLLFRACCTL_W<'_, PLLrs> {
        PLLFRACCTL_W::new(self, 29)
    }
    ///Bit 30 - PLLDITHEN0
    #[inline(always)]
    pub fn plldithen0(&mut self) -> PLLDITHEN0_W<'_, PLLrs> {
        PLLDITHEN0_W::new(self, 30)
    }
    ///Bit 31 - PLLDITHEN1
    #[inline(always)]
    pub fn plldithen1(&mut self) -> PLLDITHEN1_W<'_, PLLrs> {
        PLLDITHEN1_W::new(self, 31)
    }
}
/**This register is used to control the PLL of the HS PHY.

You can [`read`](crate::Reg::read) this register and get [`pll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#USBPHYC:PLL)*/
pub struct PLLrs;
impl crate::RegisterSpec for PLLrs {
    type Ux = u32;
}
///`read()` method returns [`pll::R`](R) reader structure
impl crate::Readable for PLLrs {}
///`write(|w| ..)` method takes [`pll::W`](W) writer structure
impl crate::Writable for PLLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL to value 0xc000_0000
impl crate::Resettable for PLLrs {
    const RESET_VALUE: u32 = 0xc000_0000;
}
