///Register `PTR1` reader
pub type R = crate::R<PTR1rs>;
///Register `PTR1` writer
pub type W = crate::W<PTR1rs>;
///Field `TDINIT0` reader - TDINIT0
pub type TDINIT0_R = crate::FieldReader<u32>;
///Field `TDINIT0` writer - TDINIT0
pub type TDINIT0_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
///Field `TDINIT1` reader - TDINIT1
pub type TDINIT1_R = crate::FieldReader;
///Field `TDINIT1` writer - TDINIT1
pub type TDINIT1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:18 - TDINIT0
    #[inline(always)]
    pub fn tdinit0(&self) -> TDINIT0_R {
        TDINIT0_R::new(self.bits & 0x0007_ffff)
    }
    ///Bits 19:26 - TDINIT1
    #[inline(always)]
    pub fn tdinit1(&self) -> TDINIT1_R {
        TDINIT1_R::new(((self.bits >> 19) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTR1")
            .field("tdinit0", &self.tdinit0())
            .field("tdinit1", &self.tdinit1())
            .finish()
    }
}
impl W {
    ///Bits 0:18 - TDINIT0
    #[inline(always)]
    pub fn tdinit0(&mut self) -> TDINIT0_W<'_, PTR1rs> {
        TDINIT0_W::new(self, 0)
    }
    ///Bits 19:26 - TDINIT1
    #[inline(always)]
    pub fn tdinit1(&mut self) -> TDINIT1_W<'_, PTR1rs> {
        TDINIT1_W::new(self, 19)
    }
}
/**DDRPHYC PT register 1

You can [`read`](crate::Reg::read) this register and get [`ptr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:PTR1)*/
pub struct PTR1rs;
impl crate::RegisterSpec for PTR1rs {
    type Ux = u32;
}
///`read()` method returns [`ptr1::R`](R) reader structure
impl crate::Readable for PTR1rs {}
///`write(|w| ..)` method takes [`ptr1::W`](W) writer structure
impl crate::Writable for PTR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PTR1 to value 0x0604_111d
impl crate::Resettable for PTR1rs {
    const RESET_VALUE: u32 = 0x0604_111d;
}
