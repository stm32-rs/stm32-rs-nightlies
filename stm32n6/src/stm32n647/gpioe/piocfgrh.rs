///Register `PIOCFGRH` reader
pub type R = crate::R<PIOCFGRHrs>;
///Register `PIOCFGRH` writer
pub type W = crate::W<PIOCFGRHrs>;
///Field `PIOCFG8` reader - Port x I/O pin y configuration
pub type PIOCFG8_R = crate::FieldReader;
///Field `PIOCFG8` writer - Port x I/O pin y configuration
pub type PIOCFG8_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PIOCFG9` reader - Port x I/O pin y configuration
pub type PIOCFG9_R = crate::FieldReader;
///Field `PIOCFG9` writer - Port x I/O pin y configuration
pub type PIOCFG9_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PIOCFG10` reader - Port x I/O pin y configuration
pub type PIOCFG10_R = crate::FieldReader;
///Field `PIOCFG10` writer - Port x I/O pin y configuration
pub type PIOCFG10_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PIOCFG11` reader - Port x I/O pin y configuration
pub type PIOCFG11_R = crate::FieldReader;
///Field `PIOCFG11` writer - Port x I/O pin y configuration
pub type PIOCFG11_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PIOCFG12` reader - Port x I/O pin y configuration
pub type PIOCFG12_R = crate::FieldReader;
///Field `PIOCFG12` writer - Port x I/O pin y configuration
pub type PIOCFG12_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PIOCFG13` reader - Port x I/O pin y configuration
pub type PIOCFG13_R = crate::FieldReader;
///Field `PIOCFG13` writer - Port x I/O pin y configuration
pub type PIOCFG13_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PIOCFG14` reader - Port x I/O pin y configuration
pub type PIOCFG14_R = crate::FieldReader;
///Field `PIOCFG14` writer - Port x I/O pin y configuration
pub type PIOCFG14_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PIOCFG15` reader - Port x I/O pin y configuration
pub type PIOCFG15_R = crate::FieldReader;
///Field `PIOCFG15` writer - Port x I/O pin y configuration
pub type PIOCFG15_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg8(&self) -> PIOCFG8_R {
        PIOCFG8_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg9(&self) -> PIOCFG9_R {
        PIOCFG9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg10(&self) -> PIOCFG10_R {
        PIOCFG10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg11(&self) -> PIOCFG11_R {
        PIOCFG11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg12(&self) -> PIOCFG12_R {
        PIOCFG12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg13(&self) -> PIOCFG13_R {
        PIOCFG13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg14(&self) -> PIOCFG14_R {
        PIOCFG14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg15(&self) -> PIOCFG15_R {
        PIOCFG15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIOCFGRH")
            .field("piocfg8", &self.piocfg8())
            .field("piocfg9", &self.piocfg9())
            .field("piocfg10", &self.piocfg10())
            .field("piocfg11", &self.piocfg11())
            .field("piocfg12", &self.piocfg12())
            .field("piocfg13", &self.piocfg13())
            .field("piocfg14", &self.piocfg14())
            .field("piocfg15", &self.piocfg15())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg8(&mut self) -> PIOCFG8_W<PIOCFGRHrs> {
        PIOCFG8_W::new(self, 0)
    }
    ///Bits 4:7 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg9(&mut self) -> PIOCFG9_W<PIOCFGRHrs> {
        PIOCFG9_W::new(self, 4)
    }
    ///Bits 8:11 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg10(&mut self) -> PIOCFG10_W<PIOCFGRHrs> {
        PIOCFG10_W::new(self, 8)
    }
    ///Bits 12:15 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg11(&mut self) -> PIOCFG11_W<PIOCFGRHrs> {
        PIOCFG11_W::new(self, 12)
    }
    ///Bits 16:19 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg12(&mut self) -> PIOCFG12_W<PIOCFGRHrs> {
        PIOCFG12_W::new(self, 16)
    }
    ///Bits 20:23 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg13(&mut self) -> PIOCFG13_W<PIOCFGRHrs> {
        PIOCFG13_W::new(self, 20)
    }
    ///Bits 24:27 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg14(&mut self) -> PIOCFG14_W<PIOCFGRHrs> {
        PIOCFG14_W::new(self, 24)
    }
    ///Bits 28:31 - Port x I/O pin y configuration
    #[inline(always)]
    pub fn piocfg15(&mut self) -> PIOCFG15_W<PIOCFGRHrs> {
        PIOCFG15_W::new(self, 28)
    }
}
/**GPIO port E PIO control high register

You can [`read`](crate::Reg::read) this register and get [`piocfgrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`piocfgrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#GPIOE:PIOCFGRH)*/
pub struct PIOCFGRHrs;
impl crate::RegisterSpec for PIOCFGRHrs {
    type Ux = u32;
}
///`read()` method returns [`piocfgrh::R`](R) reader structure
impl crate::Readable for PIOCFGRHrs {}
///`write(|w| ..)` method takes [`piocfgrh::W`](W) writer structure
impl crate::Writable for PIOCFGRHrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIOCFGRH to value 0
impl crate::Resettable for PIOCFGRHrs {}
