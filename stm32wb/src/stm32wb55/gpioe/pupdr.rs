///Register `PUPDR` reader
pub type R = crate::R<PUPDRrs>;
///Register `PUPDR` writer
pub type W = crate::W<PUPDRrs>;
///Field `PUPDR0` reader - Port x configuration bits (y = 0..15)
pub type PUPDR0_R = crate::FieldReader;
///Field `PUPDR0` writer - Port x configuration bits (y = 0..15)
pub type PUPDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PUPDR1` reader - Port x configuration bits (y = 0..15)
pub type PUPDR1_R = crate::FieldReader;
///Field `PUPDR1` writer - Port x configuration bits (y = 0..15)
pub type PUPDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PUPDR2` reader - Port x configuration bits (y = 0..15)
pub type PUPDR2_R = crate::FieldReader;
///Field `PUPDR2` writer - Port x configuration bits (y = 0..15)
pub type PUPDR2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PUPDR3` reader - Port x configuration bits (y = 0..15)
pub type PUPDR3_R = crate::FieldReader;
///Field `PUPDR3` writer - Port x configuration bits (y = 0..15)
pub type PUPDR3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PUPDR4` reader - Port x configuration bits (y = 0..15)
pub type PUPDR4_R = crate::FieldReader;
///Field `PUPDR4` writer - Port x configuration bits (y = 0..15)
pub type PUPDR4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr0(&self) -> PUPDR0_R {
        PUPDR0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr1(&self) -> PUPDR1_R {
        PUPDR1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr2(&self) -> PUPDR2_R {
        PUPDR2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr3(&self) -> PUPDR3_R {
        PUPDR3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr4(&self) -> PUPDR4_R {
        PUPDR4_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUPDR")
            .field("pupdr4", &self.pupdr4())
            .field("pupdr3", &self.pupdr3())
            .field("pupdr2", &self.pupdr2())
            .field("pupdr1", &self.pupdr1())
            .field("pupdr0", &self.pupdr0())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr0(&mut self) -> PUPDR0_W<PUPDRrs> {
        PUPDR0_W::new(self, 0)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr1(&mut self) -> PUPDR1_W<PUPDRrs> {
        PUPDR1_W::new(self, 2)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr2(&mut self) -> PUPDR2_W<PUPDRrs> {
        PUPDR2_W::new(self, 4)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr3(&mut self) -> PUPDR3_W<PUPDRrs> {
        PUPDR3_W::new(self, 6)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr4(&mut self) -> PUPDR4_W<PUPDRrs> {
        PUPDR4_W::new(self, 8)
    }
}
/**GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`pupdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOE:PUPDR)*/
pub struct PUPDRrs;
impl crate::RegisterSpec for PUPDRrs {
    type Ux = u32;
}
///`read()` method returns [`pupdr::R`](R) reader structure
impl crate::Readable for PUPDRrs {}
///`write(|w| ..)` method takes [`pupdr::W`](W) writer structure
impl crate::Writable for PUPDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PUPDR to value 0
impl crate::Resettable for PUPDRrs {
    const RESET_VALUE: u32 = 0;
}
