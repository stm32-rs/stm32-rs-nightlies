///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `PVDEN` reader - Programmable voltage detector enable
pub type PVDEN_R = crate::BitReader;
///Field `PVDEN` writer - Programmable voltage detector enable
pub type PVDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVDO` reader - Programmable voltage detect output
pub type PVDO_R = crate::BitReader;
impl R {
    ///Bit 0 - Programmable voltage detector enable
    #[inline(always)]
    pub fn pvden(&self) -> PVDEN_R {
        PVDEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Programmable voltage detect output
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("pvden", &self.pvden())
            .field("pvdo", &self.pvdo())
            .finish()
    }
}
impl W {
    ///Bit 0 - Programmable voltage detector enable
    #[inline(always)]
    pub fn pvden(&mut self) -> PVDEN_W<'_, CR2rs> {
        PVDEN_W::new(self, 0)
    }
}
/**PWR control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#PWR:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
