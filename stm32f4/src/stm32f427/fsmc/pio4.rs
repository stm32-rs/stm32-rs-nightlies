///Register `PIO4` reader
pub type R = crate::R<PIO4rs>;
///Register `PIO4` writer
pub type W = crate::W<PIO4rs>;
///Field `IOSET` reader - IOSETx
pub type IOSET_R = crate::FieldReader;
///Field `IOSET` writer - IOSETx
pub type IOSET_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `IOWAIT` reader - IOWAITx
pub type IOWAIT_R = crate::FieldReader;
///Field `IOWAIT` writer - IOWAITx
pub type IOWAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IOHOLD` reader - IOHOLDx
pub type IOHOLD_R = crate::FieldReader;
///Field `IOHOLD` writer - IOHOLDx
pub type IOHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IOHIZ` reader - IOHIZx
pub type IOHIZ_R = crate::FieldReader;
///Field `IOHIZ` writer - IOHIZx
pub type IOHIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bits 0:7 - IOSETx
    #[inline(always)]
    pub fn ioset(&self) -> IOSET_R {
        IOSET_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - IOWAITx
    #[inline(always)]
    pub fn iowait(&self) -> IOWAIT_R {
        IOWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - IOHOLDx
    #[inline(always)]
    pub fn iohold(&self) -> IOHOLD_R {
        IOHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - IOHIZx
    #[inline(always)]
    pub fn iohiz(&self) -> IOHIZ_R {
        IOHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO4")
            .field("iohiz", &self.iohiz())
            .field("iohold", &self.iohold())
            .field("iowait", &self.iowait())
            .field("ioset", &self.ioset())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - IOSETx
    #[inline(always)]
    pub fn ioset(&mut self) -> IOSET_W<'_, PIO4rs> {
        IOSET_W::new(self, 0)
    }
    ///Bits 8:15 - IOWAITx
    #[inline(always)]
    pub fn iowait(&mut self) -> IOWAIT_W<'_, PIO4rs> {
        IOWAIT_W::new(self, 8)
    }
    ///Bits 16:23 - IOHOLDx
    #[inline(always)]
    pub fn iohold(&mut self) -> IOHOLD_W<'_, PIO4rs> {
        IOHOLD_W::new(self, 16)
    }
    ///Bits 24:31 - IOHIZx
    #[inline(always)]
    pub fn iohiz(&mut self) -> IOHIZ_W<'_, PIO4rs> {
        IOHIZ_W::new(self, 24)
    }
}
/**I/O space timing register 4

You can [`read`](crate::Reg::read) this register and get [`pio4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pio4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F427.html#FSMC:PIO4)*/
pub struct PIO4rs;
impl crate::RegisterSpec for PIO4rs {
    type Ux = u32;
}
///`read()` method returns [`pio4::R`](R) reader structure
impl crate::Readable for PIO4rs {}
///`write(|w| ..)` method takes [`pio4::W`](W) writer structure
impl crate::Writable for PIO4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIO4 to value 0xfcfc_fcfc
impl crate::Resettable for PIO4rs {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
