///Register `FMT0_DR` reader
pub type R = crate::R<FMT0_DRrs>;
///Field `DR` reader - DR
pub type DR_R = crate::FieldReader<u32>;
///Field `PE` reader - PE
pub type PE_R = crate::BitReader;
///Field `V` reader - V
pub type V_R = crate::BitReader;
///Field `U` reader - U
pub type U_R = crate::BitReader;
///Field `C` reader - C
pub type C_R = crate::BitReader;
///Field `PT` reader - PT
pub type PT_R = crate::FieldReader;
impl R {
    ///Bits 0:23 - DR
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(self.bits & 0x00ff_ffff)
    }
    ///Bit 24 - PE
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - V
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - U
    #[inline(always)]
    pub fn u(&self) -> U_R {
        U_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - C
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:29 - PT
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMT0_DR")
            .field("dr", &self.dr())
            .field("pe", &self.pe())
            .field("v", &self.v())
            .field("u", &self.u())
            .field("c", &self.c())
            .field("pt", &self.pt())
            .finish()
    }
}
/**This register can take 3 different formats according to DRFMT. Here is the format when DRFMT = 00:

You can [`read`](crate::Reg::read) this register and get [`fmt0_dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPDIFRX:FMT0_DR)*/
pub struct FMT0_DRrs;
impl crate::RegisterSpec for FMT0_DRrs {
    type Ux = u32;
}
///`read()` method returns [`fmt0_dr::R`](R) reader structure
impl crate::Readable for FMT0_DRrs {}
///`reset()` method sets FMT0_DR to value 0
impl crate::Resettable for FMT0_DRrs {}
