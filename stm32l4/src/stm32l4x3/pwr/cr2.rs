///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `PVDE` reader - Power voltage detector enable
pub type PVDE_R = crate::BitReader;
///Field `PVDE` writer - Power voltage detector enable
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLS` reader - Power voltage detector level selection
pub type PLS_R = crate::FieldReader;
///Field `PLS` writer - Power voltage detector level selection
pub type PLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PVME1` reader - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V
pub type PVME1_R = crate::BitReader;
///Field `PVME1` writer - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V
pub type PVME1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVME2` reader - Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V
pub type PVME2_R = crate::BitReader;
///Field `PVME2` writer - Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V
pub type PVME2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVME3` reader - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V
pub type PVME3_R = crate::BitReader;
///Field `PVME3` writer - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V
pub type PVME3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVME4` reader - Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V
pub type PVME4_R = crate::BitReader;
///Field `PVME4` writer - Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V
pub type PVME4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IOSV` reader - VDDIO2 Independent I/Os supply valid
pub type IOSV_R = crate::BitReader;
///Field `IOSV` writer - VDDIO2 Independent I/Os supply valid
pub type IOSV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USV` reader - VDDUSB USB supply valid
pub type USV_R = crate::BitReader;
///Field `USV` writer - VDDUSB USB supply valid
pub type USV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - Power voltage detector level selection
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bit 4 - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V
    #[inline(always)]
    pub fn pvme1(&self) -> PVME1_R {
        PVME1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V
    #[inline(always)]
    pub fn pvme2(&self) -> PVME2_R {
        PVME2_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V
    #[inline(always)]
    pub fn pvme3(&self) -> PVME3_R {
        PVME3_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V
    #[inline(always)]
    pub fn pvme4(&self) -> PVME4_R {
        PVME4_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - VDDIO2 Independent I/Os supply valid
    #[inline(always)]
    pub fn iosv(&self) -> IOSV_R {
        IOSV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - VDDUSB USB supply valid
    #[inline(always)]
    pub fn usv(&self) -> USV_R {
        USV_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("usv", &self.usv())
            .field("iosv", &self.iosv())
            .field("pvme4", &self.pvme4())
            .field("pvme3", &self.pvme3())
            .field("pvme2", &self.pvme2())
            .field("pvme1", &self.pvme1())
            .field("pls", &self.pls())
            .field("pvde", &self.pvde())
            .finish()
    }
}
impl W {
    ///Bit 0 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W<'_, CR2rs> {
        PVDE_W::new(self, 0)
    }
    ///Bits 1:3 - Power voltage detector level selection
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W<'_, CR2rs> {
        PLS_W::new(self, 1)
    }
    ///Bit 4 - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V
    #[inline(always)]
    pub fn pvme1(&mut self) -> PVME1_W<'_, CR2rs> {
        PVME1_W::new(self, 4)
    }
    ///Bit 5 - Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V
    #[inline(always)]
    pub fn pvme2(&mut self) -> PVME2_W<'_, CR2rs> {
        PVME2_W::new(self, 5)
    }
    ///Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V
    #[inline(always)]
    pub fn pvme3(&mut self) -> PVME3_W<'_, CR2rs> {
        PVME3_W::new(self, 6)
    }
    ///Bit 7 - Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V
    #[inline(always)]
    pub fn pvme4(&mut self) -> PVME4_W<'_, CR2rs> {
        PVME4_W::new(self, 7)
    }
    ///Bit 9 - VDDIO2 Independent I/Os supply valid
    #[inline(always)]
    pub fn iosv(&mut self) -> IOSV_W<'_, CR2rs> {
        IOSV_W::new(self, 9)
    }
    ///Bit 10 - VDDUSB USB supply valid
    #[inline(always)]
    pub fn usv(&mut self) -> USV_W<'_, CR2rs> {
        USV_W::new(self, 10)
    }
}
/**Power control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x3.html#PWR:CR2)*/
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
