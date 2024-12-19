///Register `MODER` reader
pub type R = crate::R<MODERrs>;
///Register `MODER` writer
pub type W = crate::W<MODERrs>;
///Field `MODER0` reader - Port x configuration bits (y = 0..15)
pub type MODER0_R = crate::FieldReader;
///Field `MODER0` writer - Port x configuration bits (y = 0..15)
pub type MODER0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODER1` reader - Port x configuration bits (y = 0..15)
pub type MODER1_R = crate::FieldReader;
///Field `MODER1` writer - Port x configuration bits (y = 0..15)
pub type MODER1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODER2` reader - Port x configuration bits (y = 0..15)
pub type MODER2_R = crate::FieldReader;
///Field `MODER2` writer - Port x configuration bits (y = 0..15)
pub type MODER2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODER3` reader - Port x configuration bits (y = 0..15)
pub type MODER3_R = crate::FieldReader;
///Field `MODER3` writer - Port x configuration bits (y = 0..15)
pub type MODER3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODER4` reader - Port x configuration bits (y = 0..15)
pub type MODER4_R = crate::FieldReader;
///Field `MODER4` writer - Port x configuration bits (y = 0..15)
pub type MODER4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder0(&self) -> MODER0_R {
        MODER0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder1(&self) -> MODER1_R {
        MODER1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder2(&self) -> MODER2_R {
        MODER2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder3(&self) -> MODER3_R {
        MODER3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder4(&self) -> MODER4_R {
        MODER4_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODER")
            .field("moder4", &self.moder4())
            .field("moder3", &self.moder3())
            .field("moder2", &self.moder2())
            .field("moder1", &self.moder1())
            .field("moder0", &self.moder0())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder0(&mut self) -> MODER0_W<MODERrs> {
        MODER0_W::new(self, 0)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder1(&mut self) -> MODER1_W<MODERrs> {
        MODER1_W::new(self, 2)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder2(&mut self) -> MODER2_W<MODERrs> {
        MODER2_W::new(self, 4)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder3(&mut self) -> MODER3_W<MODERrs> {
        MODER3_W::new(self, 6)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder4(&mut self) -> MODER4_W<MODERrs> {
        MODER4_W::new(self, 8)
    }
}
/**GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`moder::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOE:MODER)*/
pub struct MODERrs;
impl crate::RegisterSpec for MODERrs {
    type Ux = u32;
}
///`read()` method returns [`moder::R`](R) reader structure
impl crate::Readable for MODERrs {}
///`write(|w| ..)` method takes [`moder::W`](W) writer structure
impl crate::Writable for MODERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MODER to value 0x03ff
impl crate::Resettable for MODERrs {
    const RESET_VALUE: u32 = 0x03ff;
}
