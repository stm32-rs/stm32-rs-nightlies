///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `INIT` writer - INIT
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAE` reader - DMAE
pub type DMAE_R = crate::BitReader;
///Field `DMAE` writer - DMAE
pub type DMAE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATATYPE` reader - DATATYPE
pub type DATATYPE_R = crate::FieldReader;
///Field `DATATYPE` writer - DATATYPE
pub type DATATYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODE` reader - MODE
pub type MODE_R = crate::BitReader;
///Field `MODE` writer - MODE
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALGO0` reader - ALGO0
pub type ALGO0_R = crate::BitReader;
///Field `ALGO0` writer - ALGO0
pub type ALGO0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NBW` reader - NBW
pub type NBW_R = crate::FieldReader;
///Field `DINNE` reader - DINNE
pub type DINNE_R = crate::BitReader;
///Field `MDMAT` reader - MDMAT
pub type MDMAT_R = crate::BitReader;
///Field `MDMAT` writer - MDMAT
pub type MDMAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAA` writer - DMAA
pub type DMAA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LKEY` reader - LKEY
pub type LKEY_R = crate::BitReader;
///Field `LKEY` writer - LKEY
pub type LKEY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALGO1` reader - ALGO1
pub type ALGO1_R = crate::BitReader;
///Field `ALGO1` writer - ALGO1
pub type ALGO1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - DMAE
    #[inline(always)]
    pub fn dmae(&self) -> DMAE_R {
        DMAE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - DATATYPE
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - MODE
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - ALGO0
    #[inline(always)]
    pub fn algo0(&self) -> ALGO0_R {
        ALGO0_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - NBW
    #[inline(always)]
    pub fn nbw(&self) -> NBW_R {
        NBW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - DINNE
    #[inline(always)]
    pub fn dinne(&self) -> DINNE_R {
        DINNE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - MDMAT
    #[inline(always)]
    pub fn mdmat(&self) -> MDMAT_R {
        MDMAT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - LKEY
    #[inline(always)]
    pub fn lkey(&self) -> LKEY_R {
        LKEY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - ALGO1
    #[inline(always)]
    pub fn algo1(&self) -> ALGO1_R {
        ALGO1_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("dmae", &self.dmae())
            .field("datatype", &self.datatype())
            .field("mode", &self.mode())
            .field("algo0", &self.algo0())
            .field("nbw", &self.nbw())
            .field("dinne", &self.dinne())
            .field("mdmat", &self.mdmat())
            .field("lkey", &self.lkey())
            .field("algo1", &self.algo1())
            .finish()
    }
}
impl W {
    ///Bit 2 - INIT
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W<'_, CRrs> {
        INIT_W::new(self, 2)
    }
    ///Bit 3 - DMAE
    #[inline(always)]
    pub fn dmae(&mut self) -> DMAE_W<'_, CRrs> {
        DMAE_W::new(self, 3)
    }
    ///Bits 4:5 - DATATYPE
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W<'_, CRrs> {
        DATATYPE_W::new(self, 4)
    }
    ///Bit 6 - MODE
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<'_, CRrs> {
        MODE_W::new(self, 6)
    }
    ///Bit 7 - ALGO0
    #[inline(always)]
    pub fn algo0(&mut self) -> ALGO0_W<'_, CRrs> {
        ALGO0_W::new(self, 7)
    }
    ///Bit 13 - MDMAT
    #[inline(always)]
    pub fn mdmat(&mut self) -> MDMAT_W<'_, CRrs> {
        MDMAT_W::new(self, 13)
    }
    ///Bit 14 - DMAA
    #[inline(always)]
    pub fn dmaa(&mut self) -> DMAA_W<'_, CRrs> {
        DMAA_W::new(self, 14)
    }
    ///Bit 16 - LKEY
    #[inline(always)]
    pub fn lkey(&mut self) -> LKEY_W<'_, CRrs> {
        LKEY_W::new(self, 16)
    }
    ///Bit 18 - ALGO1
    #[inline(always)]
    pub fn algo1(&mut self) -> ALGO1_W<'_, CRrs> {
        ALGO1_W::new(self, 18)
    }
}
/**HASH control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HASH2:CR)*/
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
