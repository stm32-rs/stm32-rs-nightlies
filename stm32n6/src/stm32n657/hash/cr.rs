///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `INIT` reader - Initialize message digest calculation
pub type INIT_R = crate::BitReader;
///Field `INIT` writer - Initialize message digest calculation
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAE` reader - DMA enable
pub type DMAE_R = crate::BitReader;
///Field `DMAE` writer - DMA enable
pub type DMAE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATATYPE` reader - Data type selection
pub type DATATYPE_R = crate::FieldReader;
///Field `DATATYPE` writer - Data type selection
pub type DATATYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODE` reader - Mode selection
pub type MODE_R = crate::BitReader;
///Field `MODE` writer - Mode selection
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NBW` reader - Number of words already pushed
pub type NBW_R = crate::FieldReader;
///Field `DINNE` reader - DIN not empty
pub type DINNE_R = crate::BitReader;
///Field `MDMAT` reader - Multiple DMA transfers
pub type MDMAT_R = crate::BitReader;
///Field `MDMAT` writer - Multiple DMA transfers
pub type MDMAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LKEY` reader - Long key selection
pub type LKEY_R = crate::BitReader;
///Field `LKEY` writer - Long key selection
pub type LKEY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALGO` reader - Algorithm selection
pub type ALGO_R = crate::FieldReader;
///Field `ALGO` writer - Algorithm selection
pub type ALGO_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 2 - Initialize message digest calculation
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DMA enable
    #[inline(always)]
    pub fn dmae(&self) -> DMAE_R {
        DMAE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Data type selection
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Mode selection
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 8:11 - Number of words already pushed
    #[inline(always)]
    pub fn nbw(&self) -> NBW_R {
        NBW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - DIN not empty
    #[inline(always)]
    pub fn dinne(&self) -> DINNE_R {
        DINNE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Multiple DMA transfers
    #[inline(always)]
    pub fn mdmat(&self) -> MDMAT_R {
        MDMAT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - Long key selection
    #[inline(always)]
    pub fn lkey(&self) -> LKEY_R {
        LKEY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:20 - Algorithm selection
    #[inline(always)]
    pub fn algo(&self) -> ALGO_R {
        ALGO_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("init", &self.init())
            .field("dmae", &self.dmae())
            .field("datatype", &self.datatype())
            .field("mode", &self.mode())
            .field("nbw", &self.nbw())
            .field("dinne", &self.dinne())
            .field("mdmat", &self.mdmat())
            .field("lkey", &self.lkey())
            .field("algo", &self.algo())
            .finish()
    }
}
impl W {
    ///Bit 2 - Initialize message digest calculation
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W<'_, CRrs> {
        INIT_W::new(self, 2)
    }
    ///Bit 3 - DMA enable
    #[inline(always)]
    pub fn dmae(&mut self) -> DMAE_W<'_, CRrs> {
        DMAE_W::new(self, 3)
    }
    ///Bits 4:5 - Data type selection
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W<'_, CRrs> {
        DATATYPE_W::new(self, 4)
    }
    ///Bit 6 - Mode selection
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<'_, CRrs> {
        MODE_W::new(self, 6)
    }
    ///Bit 13 - Multiple DMA transfers
    #[inline(always)]
    pub fn mdmat(&mut self) -> MDMAT_W<'_, CRrs> {
        MDMAT_W::new(self, 13)
    }
    ///Bit 16 - Long key selection
    #[inline(always)]
    pub fn lkey(&mut self) -> LKEY_W<'_, CRrs> {
        LKEY_W::new(self, 16)
    }
    ///Bits 17:20 - Algorithm selection
    #[inline(always)]
    pub fn algo(&mut self) -> ALGO_W<'_, CRrs> {
        ALGO_W::new(self, 17)
    }
}
/**HASH control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#HASH:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
