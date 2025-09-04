///Register `FMT0_DR_alternate1` reader
pub type R = crate::R<FMT0_DR_ALTERNATE1rs>;
///Field `PE` reader - parity error bit Contains a copy of PERR bit if PMSK = 0, otherwise it is forced to 0
pub type PE_R = crate::BitReader;
///Field `V` reader - validity bit Contains the received validity bit if VMSK = 0, otherwise it is forced to 0
pub type V_R = crate::BitReader;
///Field `U` reader - user bit Contains the received user bit, if CUMSK = 0, otherwise it is forced to 0
pub type U_R = crate::BitReader;
///Field `C` reader - channel Status bit Contains the received channel status bit, if CUMSK = 0, otherwise it is forced to 0
pub type C_R = crate::BitReader;
///Field `PT` reader - preamble type These bits indicate the preamble received. Note that if PTMSK = 1, this field is forced to zero
pub type PT_R = crate::FieldReader;
///Field `DR` reader - data value Contains the 24 received data bits, aligned on D\[23\]
pub type DR_R = crate::FieldReader<u32>;
impl R {
    ///Bit 0 - parity error bit Contains a copy of PERR bit if PMSK = 0, otherwise it is forced to 0
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - validity bit Contains the received validity bit if VMSK = 0, otherwise it is forced to 0
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - user bit Contains the received user bit, if CUMSK = 0, otherwise it is forced to 0
    #[inline(always)]
    pub fn u(&self) -> U_R {
        U_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - channel Status bit Contains the received channel status bit, if CUMSK = 0, otherwise it is forced to 0
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - preamble type These bits indicate the preamble received. Note that if PTMSK = 1, this field is forced to zero
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:31 - data value Contains the 24 received data bits, aligned on D\[23\]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMT0_DR_alternate1")
            .field("pe", &self.pe())
            .field("v", &self.v())
            .field("u", &self.u())
            .field("c", &self.c())
            .field("pt", &self.pt())
            .field("dr", &self.dr())
            .finish()
    }
}
/**SPDIFRX data input register

You can [`read`](crate::Reg::read) this register and get [`fmt0_dr_alternate1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SPDIFRX:FMT0_DR_alternate1)*/
pub struct FMT0_DR_ALTERNATE1rs;
impl crate::RegisterSpec for FMT0_DR_ALTERNATE1rs {
    type Ux = u32;
}
///`read()` method returns [`fmt0_dr_alternate1::R`](R) reader structure
impl crate::Readable for FMT0_DR_ALTERNATE1rs {}
///`reset()` method sets FMT0_DR_alternate1 to value 0
impl crate::Resettable for FMT0_DR_ALTERNATE1rs {}
