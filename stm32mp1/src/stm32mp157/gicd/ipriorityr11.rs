///Register `IPRIORITYR11` reader
pub type R = crate::R<IPRIORITYR11rs>;
///Register `IPRIORITYR11` writer
pub type W = crate::W<IPRIORITYR11rs>;
///Field `PRIORITY0` reader - PRIORITY0
pub type PRIORITY0_R = crate::FieldReader;
///Field `PRIORITY0` writer - PRIORITY0
pub type PRIORITY0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `PRIORITY1` reader - PRIORITY1
pub type PRIORITY1_R = crate::FieldReader;
///Field `PRIORITY1` writer - PRIORITY1
pub type PRIORITY1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `PRIORITY2` reader - PRIORITY2
pub type PRIORITY2_R = crate::FieldReader;
///Field `PRIORITY2` writer - PRIORITY2
pub type PRIORITY2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `PRIORITY3` reader - PRIORITY3
pub type PRIORITY3_R = crate::FieldReader;
///Field `PRIORITY3` writer - PRIORITY3
pub type PRIORITY3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 3:7 - PRIORITY0
    #[inline(always)]
    pub fn priority0(&self) -> PRIORITY0_R {
        PRIORITY0_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    ///Bits 11:15 - PRIORITY1
    #[inline(always)]
    pub fn priority1(&self) -> PRIORITY1_R {
        PRIORITY1_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    ///Bits 19:23 - PRIORITY2
    #[inline(always)]
    pub fn priority2(&self) -> PRIORITY2_R {
        PRIORITY2_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    ///Bits 27:31 - PRIORITY3
    #[inline(always)]
    pub fn priority3(&self) -> PRIORITY3_R {
        PRIORITY3_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPRIORITYR11")
            .field("priority0", &self.priority0())
            .field("priority1", &self.priority1())
            .field("priority2", &self.priority2())
            .field("priority3", &self.priority3())
            .finish()
    }
}
impl W {
    ///Bits 3:7 - PRIORITY0
    #[inline(always)]
    pub fn priority0(&mut self) -> PRIORITY0_W<IPRIORITYR11rs> {
        PRIORITY0_W::new(self, 3)
    }
    ///Bits 11:15 - PRIORITY1
    #[inline(always)]
    pub fn priority1(&mut self) -> PRIORITY1_W<IPRIORITYR11rs> {
        PRIORITY1_W::new(self, 11)
    }
    ///Bits 19:23 - PRIORITY2
    #[inline(always)]
    pub fn priority2(&mut self) -> PRIORITY2_W<IPRIORITYR11rs> {
        PRIORITY2_W::new(self, 19)
    }
    ///Bits 27:31 - PRIORITY3
    #[inline(always)]
    pub fn priority3(&mut self) -> PRIORITY3_W<IPRIORITYR11rs> {
        PRIORITY3_W::new(self, 27)
    }
}
/**GICD interrupt priority register 11

You can [`read`](crate::Reg::read) this register and get [`ipriorityr11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR11)*/
pub struct IPRIORITYR11rs;
impl crate::RegisterSpec for IPRIORITYR11rs {
    type Ux = u32;
}
///`read()` method returns [`ipriorityr11::R`](R) reader structure
impl crate::Readable for IPRIORITYR11rs {}
///`write(|w| ..)` method takes [`ipriorityr11::W`](W) writer structure
impl crate::Writable for IPRIORITYR11rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IPRIORITYR11 to value 0
impl crate::Resettable for IPRIORITYR11rs {
    const RESET_VALUE: u32 = 0;
}