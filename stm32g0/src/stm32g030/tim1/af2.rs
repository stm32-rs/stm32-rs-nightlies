///Register `AF2` reader
pub type R = crate::R<AF2rs>;
///Register `AF2` writer
pub type W = crate::W<AF2rs>;
///Field `BK2INE` reader - BRK2 BKIN input enable
pub type BK2INE_R = crate::BitReader;
///Field `BK2INE` writer - BRK2 BKIN input enable
pub type BK2INE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BK2CMP1E` reader - BRK2 COMP1 enable
pub type BK2CMP1E_R = crate::BitReader;
///Field `BK2CMP1E` writer - BRK2 COMP1 enable
pub type BK2CMP1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BK2CMP2E` reader - BRK2 COMP2 enable
pub type BK2CMP2E_R = crate::BitReader;
///Field `BK2CMP2E` writer - BRK2 COMP2 enable
pub type BK2CMP2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BK2DFBK0E` reader - BRK2 DFSDM_BREAK0 enable
pub type BK2DFBK0E_R = crate::BitReader;
///Field `BK2DFBK0E` writer - BRK2 DFSDM_BREAK0 enable
pub type BK2DFBK0E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BK2INP` reader - BRK2 BKIN input polarity
pub type BK2INP_R = crate::BitReader;
///Field `BK2INP` writer - BRK2 BKIN input polarity
pub type BK2INP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BK2CMP1P` reader - BRK2 COMP1 input polarity
pub type BK2CMP1P_R = crate::BitReader;
///Field `BK2CMP1P` writer - BRK2 COMP1 input polarity
pub type BK2CMP1P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BK2CMP2P` reader - BRK2 COMP2 input polarity
pub type BK2CMP2P_R = crate::BitReader;
///Field `BK2CMP2P` writer - BRK2 COMP2 input polarity
pub type BK2CMP2P_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - BRK2 BKIN input enable
    #[inline(always)]
    pub fn bk2ine(&self) -> BK2INE_R {
        BK2INE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BRK2 COMP1 enable
    #[inline(always)]
    pub fn bk2cmp1e(&self) -> BK2CMP1E_R {
        BK2CMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - BRK2 COMP2 enable
    #[inline(always)]
    pub fn bk2cmp2e(&self) -> BK2CMP2E_R {
        BK2CMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - BRK2 DFSDM_BREAK0 enable
    #[inline(always)]
    pub fn bk2dfbk0e(&self) -> BK2DFBK0E_R {
        BK2DFBK0E_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BRK2 BKIN input polarity
    #[inline(always)]
    pub fn bk2inp(&self) -> BK2INP_R {
        BK2INP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - BRK2 COMP1 input polarity
    #[inline(always)]
    pub fn bk2cmp1p(&self) -> BK2CMP1P_R {
        BK2CMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - BRK2 COMP2 input polarity
    #[inline(always)]
    pub fn bk2cmp2p(&self) -> BK2CMP2P_R {
        BK2CMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF2")
            .field("bk2ine", &self.bk2ine())
            .field("bk2cmp1e", &self.bk2cmp1e())
            .field("bk2cmp2e", &self.bk2cmp2e())
            .field("bk2dfbk0e", &self.bk2dfbk0e())
            .field("bk2inp", &self.bk2inp())
            .field("bk2cmp1p", &self.bk2cmp1p())
            .field("bk2cmp2p", &self.bk2cmp2p())
            .finish()
    }
}
impl W {
    ///Bit 0 - BRK2 BKIN input enable
    #[inline(always)]
    pub fn bk2ine(&mut self) -> BK2INE_W<AF2rs> {
        BK2INE_W::new(self, 0)
    }
    ///Bit 1 - BRK2 COMP1 enable
    #[inline(always)]
    pub fn bk2cmp1e(&mut self) -> BK2CMP1E_W<AF2rs> {
        BK2CMP1E_W::new(self, 1)
    }
    ///Bit 2 - BRK2 COMP2 enable
    #[inline(always)]
    pub fn bk2cmp2e(&mut self) -> BK2CMP2E_W<AF2rs> {
        BK2CMP2E_W::new(self, 2)
    }
    ///Bit 8 - BRK2 DFSDM_BREAK0 enable
    #[inline(always)]
    pub fn bk2dfbk0e(&mut self) -> BK2DFBK0E_W<AF2rs> {
        BK2DFBK0E_W::new(self, 8)
    }
    ///Bit 9 - BRK2 BKIN input polarity
    #[inline(always)]
    pub fn bk2inp(&mut self) -> BK2INP_W<AF2rs> {
        BK2INP_W::new(self, 9)
    }
    ///Bit 10 - BRK2 COMP1 input polarity
    #[inline(always)]
    pub fn bk2cmp1p(&mut self) -> BK2CMP1P_W<AF2rs> {
        BK2CMP1P_W::new(self, 10)
    }
    ///Bit 11 - BRK2 COMP2 input polarity
    #[inline(always)]
    pub fn bk2cmp2p(&mut self) -> BK2CMP2P_W<AF2rs> {
        BK2CMP2P_W::new(self, 11)
    }
}
/**DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`af2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G030.html#TIM1:AF2)*/
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
