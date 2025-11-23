///Register `PTR2` reader
pub type R = crate::R<PTR2rs>;
///Register `PTR2` writer
pub type W = crate::W<PTR2rs>;
///Field `TDINIT2` reader - TDINIT2
pub type TDINIT2_R = crate::FieldReader<u32>;
///Field `TDINIT2` writer - TDINIT2
pub type TDINIT2_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
///Field `TDINIT3` reader - TDINIT3
pub type TDINIT3_R = crate::FieldReader<u16>;
///Field `TDINIT3` writer - TDINIT3
pub type TDINIT3_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:16 - TDINIT2
    #[inline(always)]
    pub fn tdinit2(&self) -> TDINIT2_R {
        TDINIT2_R::new(self.bits & 0x0001_ffff)
    }
    ///Bits 17:26 - TDINIT3
    #[inline(always)]
    pub fn tdinit3(&self) -> TDINIT3_R {
        TDINIT3_R::new(((self.bits >> 17) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTR2")
            .field("tdinit2", &self.tdinit2())
            .field("tdinit3", &self.tdinit3())
            .finish()
    }
}
impl W {
    ///Bits 0:16 - TDINIT2
    #[inline(always)]
    pub fn tdinit2(&mut self) -> TDINIT2_W<'_, PTR2rs> {
        TDINIT2_W::new(self, 0)
    }
    ///Bits 17:26 - TDINIT3
    #[inline(always)]
    pub fn tdinit3(&mut self) -> TDINIT3_W<'_, PTR2rs> {
        TDINIT3_W::new(self, 17)
    }
}
/**DDRPHYC PT register 2

You can [`read`](crate::Reg::read) this register and get [`ptr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:PTR2)*/
pub struct PTR2rs;
impl crate::RegisterSpec for PTR2rs {
    type Ux = u32;
}
///`read()` method returns [`ptr2::R`](R) reader structure
impl crate::Readable for PTR2rs {}
///`write(|w| ..)` method takes [`ptr2::W`](W) writer structure
impl crate::Writable for PTR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PTR2 to value 0x042d_a072
impl crate::Resettable for PTR2rs {
    const RESET_VALUE: u32 = 0x042d_a072;
}
