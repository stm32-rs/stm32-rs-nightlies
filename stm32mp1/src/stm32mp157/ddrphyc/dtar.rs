///Register `DTAR` reader
pub type R = crate::R<DTARrs>;
///Register `DTAR` writer
pub type W = crate::W<DTARrs>;
///Field `DTCOL` reader - DTCOL
pub type DTCOL_R = crate::FieldReader<u16>;
///Field `DTCOL` writer - DTCOL
pub type DTCOL_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `DTROW` reader - DTROW
pub type DTROW_R = crate::FieldReader<u16>;
///Field `DTROW` writer - DTROW
pub type DTROW_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `DTBANK` reader - DTBANK
pub type DTBANK_R = crate::FieldReader;
///Field `DTBANK` writer - DTBANK
pub type DTBANK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DTMPR` reader - DTMPR
pub type DTMPR_R = crate::BitReader;
///Field `DTMPR` writer - DTMPR
pub type DTMPR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:11 - DTCOL
    #[inline(always)]
    pub fn dtcol(&self) -> DTCOL_R {
        DTCOL_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 12:27 - DTROW
    #[inline(always)]
    pub fn dtrow(&self) -> DTROW_R {
        DTROW_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    ///Bits 28:30 - DTBANK
    #[inline(always)]
    pub fn dtbank(&self) -> DTBANK_R {
        DTBANK_R::new(((self.bits >> 28) & 7) as u8)
    }
    ///Bit 31 - DTMPR
    #[inline(always)]
    pub fn dtmpr(&self) -> DTMPR_R {
        DTMPR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTAR")
            .field("dtcol", &self.dtcol())
            .field("dtrow", &self.dtrow())
            .field("dtbank", &self.dtbank())
            .field("dtmpr", &self.dtmpr())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - DTCOL
    #[inline(always)]
    pub fn dtcol(&mut self) -> DTCOL_W<'_, DTARrs> {
        DTCOL_W::new(self, 0)
    }
    ///Bits 12:27 - DTROW
    #[inline(always)]
    pub fn dtrow(&mut self) -> DTROW_W<'_, DTARrs> {
        DTROW_W::new(self, 12)
    }
    ///Bits 28:30 - DTBANK
    #[inline(always)]
    pub fn dtbank(&mut self) -> DTBANK_W<'_, DTARrs> {
        DTBANK_W::new(self, 28)
    }
    ///Bit 31 - DTMPR
    #[inline(always)]
    pub fn dtmpr(&mut self) -> DTMPR_W<'_, DTARrs> {
        DTMPR_W::new(self, 31)
    }
}
/**DDRPHYC DTA register

You can [`read`](crate::Reg::read) this register and get [`dtar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DTAR)*/
pub struct DTARrs;
impl crate::RegisterSpec for DTARrs {
    type Ux = u32;
}
///`read()` method returns [`dtar::R`](R) reader structure
impl crate::Readable for DTARrs {}
///`write(|w| ..)` method takes [`dtar::W`](W) writer structure
impl crate::Writable for DTARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DTAR to value 0
impl crate::Resettable for DTARrs {}
