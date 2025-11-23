///Register `DEVR2` reader
pub type R = crate::R<DEVR2rs>;
///Register `DEVR2` writer
pub type W = crate::W<DEVR2rs>;
///Field `DA` reader - Assigned I3C dynamic address to target x (when the I3C acts as controller)
pub type DA_R = crate::FieldReader;
///Field `DA` writer - Assigned I3C dynamic address to target x (when the I3C acts as controller)
pub type DA_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `IBIACK` reader - IBI request acknowledge (when the I3C acts as controller)
pub type IBIACK_R = crate::BitReader;
///Field `IBIACK` writer - IBI request acknowledge (when the I3C acts as controller)
pub type IBIACK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRACK` reader - Controller-role request acknowledge (when the I3C acts as controller)
pub type CRACK_R = crate::BitReader;
///Field `CRACK` writer - Controller-role request acknowledge (when the I3C acts as controller)
pub type CRACK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IBIDEN` reader - IBI data enable (when the I3C acts as controller)
pub type IBIDEN_R = crate::BitReader;
///Field `IBIDEN` writer - IBI data enable (when the I3C acts as controller)
pub type IBIDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUSP` reader - Suspend/stop I3C transfer on received IBI (when the I3C acts as controller)
pub type SUSP_R = crate::BitReader;
///Field `SUSP` writer - Suspend/stop I3C transfer on received IBI (when the I3C acts as controller)
pub type SUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIS` reader - DA\[6:0\] write disabled (when the I3C acts as controller)
pub type DIS_R = crate::BitReader;
impl R {
    ///Bits 1:7 - Assigned I3C dynamic address to target x (when the I3C acts as controller)
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    ///Bit 16 - IBI request acknowledge (when the I3C acts as controller)
    #[inline(always)]
    pub fn ibiack(&self) -> IBIACK_R {
        IBIACK_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Controller-role request acknowledge (when the I3C acts as controller)
    #[inline(always)]
    pub fn crack(&self) -> CRACK_R {
        CRACK_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - IBI data enable (when the I3C acts as controller)
    #[inline(always)]
    pub fn ibiden(&self) -> IBIDEN_R {
        IBIDEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Suspend/stop I3C transfer on received IBI (when the I3C acts as controller)
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 31 - DA\[6:0\] write disabled (when the I3C acts as controller)
    #[inline(always)]
    pub fn dis(&self) -> DIS_R {
        DIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVR2")
            .field("da", &self.da())
            .field("ibiack", &self.ibiack())
            .field("crack", &self.crack())
            .field("ibiden", &self.ibiden())
            .field("susp", &self.susp())
            .field("dis", &self.dis())
            .finish()
    }
}
impl W {
    ///Bits 1:7 - Assigned I3C dynamic address to target x (when the I3C acts as controller)
    #[inline(always)]
    pub fn da(&mut self) -> DA_W<'_, DEVR2rs> {
        DA_W::new(self, 1)
    }
    ///Bit 16 - IBI request acknowledge (when the I3C acts as controller)
    #[inline(always)]
    pub fn ibiack(&mut self) -> IBIACK_W<'_, DEVR2rs> {
        IBIACK_W::new(self, 16)
    }
    ///Bit 17 - Controller-role request acknowledge (when the I3C acts as controller)
    #[inline(always)]
    pub fn crack(&mut self) -> CRACK_W<'_, DEVR2rs> {
        CRACK_W::new(self, 17)
    }
    ///Bit 18 - IBI data enable (when the I3C acts as controller)
    #[inline(always)]
    pub fn ibiden(&mut self) -> IBIDEN_W<'_, DEVR2rs> {
        IBIDEN_W::new(self, 18)
    }
    ///Bit 19 - Suspend/stop I3C transfer on received IBI (when the I3C acts as controller)
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<'_, DEVR2rs> {
        SUSP_W::new(self, 19)
    }
}
/**I3C device 2 characteristics register

You can [`read`](crate::Reg::read) this register and get [`devr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#I3C1:DEVR2)*/
pub struct DEVR2rs;
impl crate::RegisterSpec for DEVR2rs {
    type Ux = u32;
}
///`read()` method returns [`devr2::R`](R) reader structure
impl crate::Readable for DEVR2rs {}
///`write(|w| ..)` method takes [`devr2::W`](W) writer structure
impl crate::Writable for DEVR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DEVR2 to value 0
impl crate::Resettable for DEVR2rs {}
