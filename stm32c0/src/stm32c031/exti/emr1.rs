///Register `EMR1` reader
pub type R = crate::R<EMR1rs>;
///Register `EMR1` writer
pub type W = crate::W<EMR1rs>;
///Field `EM` reader - CPU wakeup with event generation mask
pub type EM_R = crate::FieldReader<u16>;
///Field `EM` writer - CPU wakeup with event generation mask
pub type EM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `EM19` reader - EM19
pub type EM19_R = crate::BitReader;
///Field `EM19` writer - EM19
pub type EM19_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM23` reader - EM23
pub type EM23_R = crate::BitReader;
///Field `EM23` writer - EM23
pub type EM23_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM25` reader - EM25
pub type EM25_R = crate::BitReader;
///Field `EM25` writer - EM25
pub type EM25_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM31` reader - EM31
pub type EM31_R = crate::BitReader;
///Field `EM31` writer - EM31
pub type EM31_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - CPU wakeup with event generation mask
    #[inline(always)]
    pub fn em(&self) -> EM_R {
        EM_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 19 - EM19
    #[inline(always)]
    pub fn em19(&self) -> EM19_R {
        EM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 23 - EM23
    #[inline(always)]
    pub fn em23(&self) -> EM23_R {
        EM23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - EM25
    #[inline(always)]
    pub fn em25(&self) -> EM25_R {
        EM25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 31 - EM31
    #[inline(always)]
    pub fn em31(&self) -> EM31_R {
        EM31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMR1")
            .field("em", &self.em())
            .field("em19", &self.em19())
            .field("em23", &self.em23())
            .field("em25", &self.em25())
            .field("em31", &self.em31())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - CPU wakeup with event generation mask
    #[inline(always)]
    pub fn em(&mut self) -> EM_W<EMR1rs> {
        EM_W::new(self, 0)
    }
    ///Bit 19 - EM19
    #[inline(always)]
    pub fn em19(&mut self) -> EM19_W<EMR1rs> {
        EM19_W::new(self, 19)
    }
    ///Bit 23 - EM23
    #[inline(always)]
    pub fn em23(&mut self) -> EM23_W<EMR1rs> {
        EM23_W::new(self, 23)
    }
    ///Bit 25 - EM25
    #[inline(always)]
    pub fn em25(&mut self) -> EM25_W<EMR1rs> {
        EM25_W::new(self, 25)
    }
    ///Bit 31 - EM31
    #[inline(always)]
    pub fn em31(&mut self) -> EM31_W<EMR1rs> {
        EM31_W::new(self, 31)
    }
}
/**EXTI CPU wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`emr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#EXTI:EMR1)*/
pub struct EMR1rs;
impl crate::RegisterSpec for EMR1rs {
    type Ux = u32;
}
///`read()` method returns [`emr1::R`](R) reader structure
impl crate::Readable for EMR1rs {}
///`write(|w| ..)` method takes [`emr1::W`](W) writer structure
impl crate::Writable for EMR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EMR1 to value 0
impl crate::Resettable for EMR1rs {
    const RESET_VALUE: u32 = 0;
}
