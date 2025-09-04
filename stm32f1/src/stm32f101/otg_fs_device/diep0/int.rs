///Register `INT` reader
pub type R = crate::R<INTrs>;
///Register `INT` writer
pub type W = crate::W<INTrs>;
///Field `XFRC` reader - XFRC
pub type XFRC_R = crate::BitReader;
///Field `XFRC` writer - XFRC
pub type XFRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDISD` reader - EPDISD
pub type EPDISD_R = crate::BitReader;
///Field `EPDISD` writer - EPDISD
pub type EPDISD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOC` reader - TOC
pub type TOC_R = crate::BitReader;
///Field `TOC` writer - TOC
pub type TOC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITTXFE` reader - ITTXFE
pub type ITTXFE_R = crate::BitReader;
///Field `ITTXFE` writer - ITTXFE
pub type ITTXFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INEPNE` reader - INEPNE
pub type INEPNE_R = crate::BitReader;
///Field `INEPNE` writer - INEPNE
pub type INEPNE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFE` reader - TXFE
pub type TXFE_R = crate::BitReader;
impl R {
    ///Bit 0 - XFRC
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EPDISD
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - TOC
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ITTXFE
    #[inline(always)]
    pub fn ittxfe(&self) -> ITTXFE_R {
        ITTXFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - INEPNE
    #[inline(always)]
    pub fn inepne(&self) -> INEPNE_R {
        INEPNE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TXFE
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT")
            .field("txfe", &self.txfe())
            .field("inepne", &self.inepne())
            .field("ittxfe", &self.ittxfe())
            .field("toc", &self.toc())
            .field("epdisd", &self.epdisd())
            .field("xfrc", &self.xfrc())
            .finish()
    }
}
impl W {
    ///Bit 0 - XFRC
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W<INTrs> {
        XFRC_W::new(self, 0)
    }
    ///Bit 1 - EPDISD
    #[inline(always)]
    pub fn epdisd(&mut self) -> EPDISD_W<INTrs> {
        EPDISD_W::new(self, 1)
    }
    ///Bit 3 - TOC
    #[inline(always)]
    pub fn toc(&mut self) -> TOC_W<INTrs> {
        TOC_W::new(self, 3)
    }
    ///Bit 4 - ITTXFE
    #[inline(always)]
    pub fn ittxfe(&mut self) -> ITTXFE_W<INTrs> {
        ITTXFE_W::new(self, 4)
    }
    ///Bit 6 - INEPNE
    #[inline(always)]
    pub fn inepne(&mut self) -> INEPNE_W<INTrs> {
        INEPNE_W::new(self, 6)
    }
}
/**device endpoint-x interrupt register

You can [`read`](crate::Reg::read) this register and get [`int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INTrs;
impl crate::RegisterSpec for INTrs {
    type Ux = u32;
}
///`read()` method returns [`int::R`](R) reader structure
impl crate::Readable for INTrs {}
///`write(|w| ..)` method takes [`int::W`](W) writer structure
impl crate::Writable for INTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INT to value 0x80
impl crate::Resettable for INTrs {
    const RESET_VALUE: u32 = 0x80;
}
