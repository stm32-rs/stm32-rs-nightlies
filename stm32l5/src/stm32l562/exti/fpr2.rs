///Register `FPR2` reader
pub type R = crate::R<FPR2rs>;
///Register `FPR2` writer
pub type W = crate::W<FPR2rs>;
///Field `FPIF35` reader - FPIF35
pub type FPIF35_R = crate::BitReader;
///Field `FPIF35` writer - FPIF35
pub type FPIF35_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF36` reader - FPIF36
pub type FPIF36_R = crate::BitReader;
///Field `FPIF36` writer - FPIF36
pub type FPIF36_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF37` reader - FPIF37
pub type FPIF37_R = crate::BitReader;
///Field `FPIF37` writer - FPIF37
pub type FPIF37_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPIF38` reader - FPIF38
pub type FPIF38_R = crate::BitReader;
///Field `FPIF38` writer - FPIF38
pub type FPIF38_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - FPIF35
    #[inline(always)]
    pub fn fpif35(&self) -> FPIF35_R {
        FPIF35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - FPIF36
    #[inline(always)]
    pub fn fpif36(&self) -> FPIF36_R {
        FPIF36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - FPIF37
    #[inline(always)]
    pub fn fpif37(&self) -> FPIF37_R {
        FPIF37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - FPIF38
    #[inline(always)]
    pub fn fpif38(&self) -> FPIF38_R {
        FPIF38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPR2")
            .field("fpif35", &self.fpif35())
            .field("fpif36", &self.fpif36())
            .field("fpif37", &self.fpif37())
            .field("fpif38", &self.fpif38())
            .finish()
    }
}
impl W {
    ///Bit 3 - FPIF35
    #[inline(always)]
    pub fn fpif35(&mut self) -> FPIF35_W<'_, FPR2rs> {
        FPIF35_W::new(self, 3)
    }
    ///Bit 4 - FPIF36
    #[inline(always)]
    pub fn fpif36(&mut self) -> FPIF36_W<'_, FPR2rs> {
        FPIF36_W::new(self, 4)
    }
    ///Bit 5 - FPIF37
    #[inline(always)]
    pub fn fpif37(&mut self) -> FPIF37_W<'_, FPR2rs> {
        FPIF37_W::new(self, 5)
    }
    ///Bit 6 - FPIF38
    #[inline(always)]
    pub fn fpif38(&mut self) -> FPIF38_W<'_, FPR2rs> {
        FPIF38_W::new(self, 6)
    }
}
/**EXTI falling edge pending register

You can [`read`](crate::Reg::read) this register and get [`fpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#EXTI:FPR2)*/
pub struct FPR2rs;
impl crate::RegisterSpec for FPR2rs {
    type Ux = u32;
}
///`read()` method returns [`fpr2::R`](R) reader structure
impl crate::Readable for FPR2rs {}
///`write(|w| ..)` method takes [`fpr2::W`](W) writer structure
impl crate::Writable for FPR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FPR2 to value 0
impl crate::Resettable for FPR2rs {}
