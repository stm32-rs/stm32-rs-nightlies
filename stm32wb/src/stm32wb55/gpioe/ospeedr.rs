///Register `OSPEEDR` reader
pub type R = crate::R<OSPEEDRrs>;
///Register `OSPEEDR` writer
pub type W = crate::W<OSPEEDRrs>;
///Field `OSPEEDR0` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR0_R = crate::FieldReader;
///Field `OSPEEDR0` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR1` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR1_R = crate::FieldReader;
///Field `OSPEEDR1` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR2` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR2_R = crate::FieldReader;
///Field `OSPEEDR2` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR3` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR3_R = crate::FieldReader;
///Field `OSPEEDR3` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR4` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR4_R = crate::FieldReader;
///Field `OSPEEDR4` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr0(&self) -> OSPEEDR0_R {
        OSPEEDR0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr1(&self) -> OSPEEDR1_R {
        OSPEEDR1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr2(&self) -> OSPEEDR2_R {
        OSPEEDR2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr3(&self) -> OSPEEDR3_R {
        OSPEEDR3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr4(&self) -> OSPEEDR4_R {
        OSPEEDR4_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSPEEDR")
            .field("ospeedr4", &self.ospeedr4())
            .field("ospeedr3", &self.ospeedr3())
            .field("ospeedr2", &self.ospeedr2())
            .field("ospeedr1", &self.ospeedr1())
            .field("ospeedr0", &self.ospeedr0())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn ospeedr0(&mut self) -> OSPEEDR0_W<OSPEEDRrs> {
        OSPEEDR0_W::new(self, 0)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn ospeedr1(&mut self) -> OSPEEDR1_W<OSPEEDRrs> {
        OSPEEDR1_W::new(self, 2)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn ospeedr2(&mut self) -> OSPEEDR2_W<OSPEEDRrs> {
        OSPEEDR2_W::new(self, 4)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn ospeedr3(&mut self) -> OSPEEDR3_W<OSPEEDRrs> {
        OSPEEDR3_W::new(self, 6)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn ospeedr4(&mut self) -> OSPEEDR4_W<OSPEEDRrs> {
        OSPEEDR4_W::new(self, 8)
    }
}
/**GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`ospeedr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOE:OSPEEDR)*/
pub struct OSPEEDRrs;
impl crate::RegisterSpec for OSPEEDRrs {
    type Ux = u32;
}
///`read()` method returns [`ospeedr::R`](R) reader structure
impl crate::Readable for OSPEEDRrs {}
///`write(|w| ..)` method takes [`ospeedr::W`](W) writer structure
impl crate::Writable for OSPEEDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OSPEEDR to value 0xc0
impl crate::Resettable for OSPEEDRrs {
    const RESET_VALUE: u32 = 0xc0;
}
