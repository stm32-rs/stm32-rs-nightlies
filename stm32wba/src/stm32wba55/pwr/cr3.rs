///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
///Field `REGSEL` reader - Regulator selection
pub type REGSEL_R = crate::BitReader;
///Field `REGSEL` writer - Regulator selection
pub type REGSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSTEN` reader - Fast soft start
pub type FSTEN_R = crate::BitReader;
///Field `FSTEN` writer - Fast soft start
pub type FSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - Regulator selection
    #[inline(always)]
    pub fn regsel(&self) -> REGSEL_R {
        REGSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Fast soft start
    #[inline(always)]
    pub fn fsten(&self) -> FSTEN_R {
        FSTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("regsel", &self.regsel())
            .field("fsten", &self.fsten())
            .finish()
    }
}
impl W {
    ///Bit 1 - Regulator selection
    #[inline(always)]
    pub fn regsel(&mut self) -> REGSEL_W<'_, CR3rs> {
        REGSEL_W::new(self, 1)
    }
    ///Bit 2 - Fast soft start
    #[inline(always)]
    pub fn fsten(&mut self) -> FSTEN_W<'_, CR3rs> {
        FSTEN_W::new(self, 2)
    }
}
/**PWR control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#PWR:CR3)*/
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
///`read()` method returns [`cr3::R`](R) reader structure
impl crate::Readable for CR3rs {}
///`write(|w| ..)` method takes [`cr3::W`](W) writer structure
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR3 to value 0
impl crate::Resettable for CR3rs {}
