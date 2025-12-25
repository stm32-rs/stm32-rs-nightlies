///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `PE` reader - PE
pub type PE_R = crate::BitReader;
///Field `PE` writer - PE
pub type PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXIE` reader - TXIE
pub type TXIE_R = crate::BitReader;
///Field `TXIE` writer - TXIE
pub type TXIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXIE` reader - RXIE
pub type RXIE_R = crate::BitReader;
///Field `RXIE` writer - RXIE
pub type RXIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADDRIE` reader - ADDRIE
pub type ADDRIE_R = crate::BitReader;
///Field `ADDRIE` writer - ADDRIE
pub type ADDRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NACKIE` reader - NACKIE
pub type NACKIE_R = crate::BitReader;
///Field `NACKIE` writer - NACKIE
pub type NACKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOPIE` reader - STOPIE
pub type STOPIE_R = crate::BitReader;
///Field `STOPIE` writer - STOPIE
pub type STOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - TCIE
pub type TCIE_R = crate::BitReader;
///Field `TCIE` writer - TCIE
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRIE` reader - ERRIE
pub type ERRIE_R = crate::BitReader;
///Field `ERRIE` writer - ERRIE
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DNF` reader - DNF
pub type DNF_R = crate::FieldReader;
///Field `DNF` writer - DNF
pub type DNF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ANFOFF` reader - ANFOFF
pub type ANFOFF_R = crate::BitReader;
///Field `ANFOFF` writer - ANFOFF
pub type ANFOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXDMAEN` reader - TXDMAEN
pub type TXDMAEN_R = crate::BitReader;
///Field `TXDMAEN` writer - TXDMAEN
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXDMAEN` reader - RXDMAEN
pub type RXDMAEN_R = crate::BitReader;
///Field `RXDMAEN` writer - RXDMAEN
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SBC` reader - SBC
pub type SBC_R = crate::BitReader;
///Field `SBC` writer - SBC
pub type SBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOSTRETCH` reader - NOSTRETCH
pub type NOSTRETCH_R = crate::BitReader;
///Field `NOSTRETCH` writer - NOSTRETCH
pub type NOSTRETCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUPEN` reader - WUPEN
pub type WUPEN_R = crate::BitReader;
///Field `WUPEN` writer - WUPEN
pub type WUPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GCEN` reader - GCEN
pub type GCEN_R = crate::BitReader;
///Field `GCEN` writer - GCEN
pub type GCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMBHEN` reader - SMBHEN
pub type SMBHEN_R = crate::BitReader;
///Field `SMBHEN` writer - SMBHEN
pub type SMBHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMBDEN` reader - SMBDEN
pub type SMBDEN_R = crate::BitReader;
///Field `SMBDEN` writer - SMBDEN
pub type SMBDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALERTEN` reader - ALERTEN
pub type ALERTEN_R = crate::BitReader;
///Field `ALERTEN` writer - ALERTEN
pub type ALERTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PECEN` reader - PECEN
pub type PECEN_R = crate::BitReader;
///Field `PECEN` writer - PECEN
pub type PECEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - PE
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TXIE
    #[inline(always)]
    pub fn txie(&self) -> TXIE_R {
        TXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RXIE
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ADDRIE
    #[inline(always)]
    pub fn addrie(&self) -> ADDRIE_R {
        ADDRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NACKIE
    #[inline(always)]
    pub fn nackie(&self) -> NACKIE_R {
        NACKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - STOPIE
    #[inline(always)]
    pub fn stopie(&self) -> STOPIE_R {
        STOPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TCIE
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - ERRIE
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - DNF
    #[inline(always)]
    pub fn dnf(&self) -> DNF_R {
        DNF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - ANFOFF
    #[inline(always)]
    pub fn anfoff(&self) -> ANFOFF_R {
        ANFOFF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - TXDMAEN
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - RXDMAEN
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SBC
    #[inline(always)]
    pub fn sbc(&self) -> SBC_R {
        SBC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - NOSTRETCH
    #[inline(always)]
    pub fn nostretch(&self) -> NOSTRETCH_R {
        NOSTRETCH_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - WUPEN
    #[inline(always)]
    pub fn wupen(&self) -> WUPEN_R {
        WUPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - GCEN
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SMBHEN
    #[inline(always)]
    pub fn smbhen(&self) -> SMBHEN_R {
        SMBHEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SMBDEN
    #[inline(always)]
    pub fn smbden(&self) -> SMBDEN_R {
        SMBDEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ALERTEN
    #[inline(always)]
    pub fn alerten(&self) -> ALERTEN_R {
        ALERTEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - PECEN
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("pe", &self.pe())
            .field("txie", &self.txie())
            .field("rxie", &self.rxie())
            .field("addrie", &self.addrie())
            .field("nackie", &self.nackie())
            .field("stopie", &self.stopie())
            .field("tcie", &self.tcie())
            .field("errie", &self.errie())
            .field("dnf", &self.dnf())
            .field("anfoff", &self.anfoff())
            .field("txdmaen", &self.txdmaen())
            .field("rxdmaen", &self.rxdmaen())
            .field("sbc", &self.sbc())
            .field("nostretch", &self.nostretch())
            .field("wupen", &self.wupen())
            .field("gcen", &self.gcen())
            .field("smbhen", &self.smbhen())
            .field("smbden", &self.smbden())
            .field("alerten", &self.alerten())
            .field("pecen", &self.pecen())
            .finish()
    }
}
impl W {
    ///Bit 0 - PE
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W<'_, CR1rs> {
        PE_W::new(self, 0)
    }
    ///Bit 1 - TXIE
    #[inline(always)]
    pub fn txie(&mut self) -> TXIE_W<'_, CR1rs> {
        TXIE_W::new(self, 1)
    }
    ///Bit 2 - RXIE
    #[inline(always)]
    pub fn rxie(&mut self) -> RXIE_W<'_, CR1rs> {
        RXIE_W::new(self, 2)
    }
    ///Bit 3 - ADDRIE
    #[inline(always)]
    pub fn addrie(&mut self) -> ADDRIE_W<'_, CR1rs> {
        ADDRIE_W::new(self, 3)
    }
    ///Bit 4 - NACKIE
    #[inline(always)]
    pub fn nackie(&mut self) -> NACKIE_W<'_, CR1rs> {
        NACKIE_W::new(self, 4)
    }
    ///Bit 5 - STOPIE
    #[inline(always)]
    pub fn stopie(&mut self) -> STOPIE_W<'_, CR1rs> {
        STOPIE_W::new(self, 5)
    }
    ///Bit 6 - TCIE
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, CR1rs> {
        TCIE_W::new(self, 6)
    }
    ///Bit 7 - ERRIE
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<'_, CR1rs> {
        ERRIE_W::new(self, 7)
    }
    ///Bits 8:11 - DNF
    #[inline(always)]
    pub fn dnf(&mut self) -> DNF_W<'_, CR1rs> {
        DNF_W::new(self, 8)
    }
    ///Bit 12 - ANFOFF
    #[inline(always)]
    pub fn anfoff(&mut self) -> ANFOFF_W<'_, CR1rs> {
        ANFOFF_W::new(self, 12)
    }
    ///Bit 14 - TXDMAEN
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<'_, CR1rs> {
        TXDMAEN_W::new(self, 14)
    }
    ///Bit 15 - RXDMAEN
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<'_, CR1rs> {
        RXDMAEN_W::new(self, 15)
    }
    ///Bit 16 - SBC
    #[inline(always)]
    pub fn sbc(&mut self) -> SBC_W<'_, CR1rs> {
        SBC_W::new(self, 16)
    }
    ///Bit 17 - NOSTRETCH
    #[inline(always)]
    pub fn nostretch(&mut self) -> NOSTRETCH_W<'_, CR1rs> {
        NOSTRETCH_W::new(self, 17)
    }
    ///Bit 18 - WUPEN
    #[inline(always)]
    pub fn wupen(&mut self) -> WUPEN_W<'_, CR1rs> {
        WUPEN_W::new(self, 18)
    }
    ///Bit 19 - GCEN
    #[inline(always)]
    pub fn gcen(&mut self) -> GCEN_W<'_, CR1rs> {
        GCEN_W::new(self, 19)
    }
    ///Bit 20 - SMBHEN
    #[inline(always)]
    pub fn smbhen(&mut self) -> SMBHEN_W<'_, CR1rs> {
        SMBHEN_W::new(self, 20)
    }
    ///Bit 21 - SMBDEN
    #[inline(always)]
    pub fn smbden(&mut self) -> SMBDEN_W<'_, CR1rs> {
        SMBDEN_W::new(self, 21)
    }
    ///Bit 22 - ALERTEN
    #[inline(always)]
    pub fn alerten(&mut self) -> ALERTEN_W<'_, CR1rs> {
        ALERTEN_W::new(self, 22)
    }
    ///Bit 23 - PECEN
    #[inline(always)]
    pub fn pecen(&mut self) -> PECEN_W<'_, CR1rs> {
        PECEN_W::new(self, 23)
    }
}
/**Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2xi2c_pclk+6xi2c_ker_ck.

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#I2C1:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {}
