///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `PVDE` reader - Power voltage detector enable
pub type PVDE_R = crate::BitReader;
///Field `PVDE` writer - Power voltage detector enable
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVDFT` reader - Power voltage detector falling threshold selection
pub type PVDFT_R = crate::FieldReader;
///Field `PVDFT` writer - Power voltage detector falling threshold selection
pub type PVDFT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PVDRT` reader - Power voltage detector rising threshold selection
pub type PVDRT_R = crate::FieldReader;
///Field `PVDRT` writer - Power voltage detector rising threshold selection
pub type PVDRT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PVMENDAC` reader - PVMENDAC
pub type PVMENDAC_R = crate::BitReader;
///Field `PVMENDAC` writer - PVMENDAC
pub type PVMENDAC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVMENUSB` reader - PVMENUSB
pub type PVMENUSB_R = crate::BitReader;
///Field `PVMENUSB` writer - PVMENUSB
pub type PVMENUSB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IOSV` reader - IOSV
pub type IOSV_R = crate::BitReader;
///Field `IOSV` writer - IOSV
pub type IOSV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USV` reader - USV
pub type USV_R = crate::BitReader;
///Field `USV` writer - USV
pub type USV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - Power voltage detector falling threshold selection
    #[inline(always)]
    pub fn pvdft(&self) -> PVDFT_R {
        PVDFT_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bits 4:6 - Power voltage detector rising threshold selection
    #[inline(always)]
    pub fn pvdrt(&self) -> PVDRT_R {
        PVDRT_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - PVMENDAC
    #[inline(always)]
    pub fn pvmendac(&self) -> PVMENDAC_R {
        PVMENDAC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PVMENUSB
    #[inline(always)]
    pub fn pvmenusb(&self) -> PVMENUSB_R {
        PVMENUSB_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - IOSV
    #[inline(always)]
    pub fn iosv(&self) -> IOSV_R {
        IOSV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - USV
    #[inline(always)]
    pub fn usv(&self) -> USV_R {
        USV_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("pvde", &self.pvde())
            .field("pvdft", &self.pvdft())
            .field("pvdrt", &self.pvdrt())
            .field("pvmendac", &self.pvmendac())
            .field("pvmenusb", &self.pvmenusb())
            .field("iosv", &self.iosv())
            .field("usv", &self.usv())
            .finish()
    }
}
impl W {
    ///Bit 0 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W<'_, CR2rs> {
        PVDE_W::new(self, 0)
    }
    ///Bits 1:3 - Power voltage detector falling threshold selection
    #[inline(always)]
    pub fn pvdft(&mut self) -> PVDFT_W<'_, CR2rs> {
        PVDFT_W::new(self, 1)
    }
    ///Bits 4:6 - Power voltage detector rising threshold selection
    #[inline(always)]
    pub fn pvdrt(&mut self) -> PVDRT_W<'_, CR2rs> {
        PVDRT_W::new(self, 4)
    }
    ///Bit 7 - PVMENDAC
    #[inline(always)]
    pub fn pvmendac(&mut self) -> PVMENDAC_W<'_, CR2rs> {
        PVMENDAC_W::new(self, 7)
    }
    ///Bit 8 - PVMENUSB
    #[inline(always)]
    pub fn pvmenusb(&mut self) -> PVMENUSB_W<'_, CR2rs> {
        PVMENUSB_W::new(self, 8)
    }
    ///Bit 9 - IOSV
    #[inline(always)]
    pub fn iosv(&mut self) -> IOSV_W<'_, CR2rs> {
        IOSV_W::new(self, 9)
    }
    ///Bit 10 - USV
    #[inline(always)]
    pub fn usv(&mut self) -> USV_W<'_, CR2rs> {
        USV_W::new(self, 10)
    }
}
/**Power control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B1.html#PWR:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
