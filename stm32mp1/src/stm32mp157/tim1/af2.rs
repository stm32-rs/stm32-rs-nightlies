///Register `AF2` reader
pub type R = crate::R<AF2rs>;
///Register `AF2` writer
pub type W = crate::W<AF2rs>;
///Field `BK2INE` reader - BK2INE
pub type BK2INE_R = crate::BitReader;
///Field `BK2INE` writer - BK2INE
pub type BK2INE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BK2DF1BK1E` reader - BK2DF1BK1E
pub type BK2DF1BK1E_R = crate::BitReader;
///Field `BK2DF1BK1E` writer - BK2DF1BK1E
pub type BK2DF1BK1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BK2INP` reader - BK2INP
pub type BK2INP_R = crate::BitReader;
///Field `BK2INP` writer - BK2INP
pub type BK2INP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - BK2INE
    #[inline(always)]
    pub fn bk2ine(&self) -> BK2INE_R {
        BK2INE_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - BK2DF1BK1E
    #[inline(always)]
    pub fn bk2df1bk1e(&self) -> BK2DF1BK1E_R {
        BK2DF1BK1E_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BK2INP
    #[inline(always)]
    pub fn bk2inp(&self) -> BK2INP_R {
        BK2INP_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF2")
            .field("bk2ine", &self.bk2ine())
            .field("bk2df1bk1e", &self.bk2df1bk1e())
            .field("bk2inp", &self.bk2inp())
            .finish()
    }
}
impl W {
    ///Bit 0 - BK2INE
    #[inline(always)]
    pub fn bk2ine(&mut self) -> BK2INE_W<AF2rs> {
        BK2INE_W::new(self, 0)
    }
    ///Bit 8 - BK2DF1BK1E
    #[inline(always)]
    pub fn bk2df1bk1e(&mut self) -> BK2DF1BK1E_W<AF2rs> {
        BK2DF1BK1E_W::new(self, 8)
    }
    ///Bit 9 - BK2INP
    #[inline(always)]
    pub fn bk2inp(&mut self) -> BK2INP_W<AF2rs> {
        BK2INP_W::new(self, 9)
    }
}
/**TIM1 Alternate function register 2

You can [`read`](crate::Reg::read) this register and get [`af2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM1:AF2)*/
pub struct AF2rs;
impl crate::RegisterSpec for AF2rs {
    type Ux = u32;
}
///`read()` method returns [`af2::R`](R) reader structure
impl crate::Readable for AF2rs {}
///`write(|w| ..)` method takes [`af2::W`](W) writer structure
impl crate::Writable for AF2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AF2 to value 0x01
impl crate::Resettable for AF2rs {
    const RESET_VALUE: u32 = 0x01;
}
